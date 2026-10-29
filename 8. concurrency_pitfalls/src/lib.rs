use std::sync::{Arc, Barrier, Mutex, TryLockError};
use std::thread;

pub struct Account {
    // IDs must be unique to define a global order.
    id: usize,
    balance: Mutex<u64>,
}

impl Account {
    pub fn new(id: usize, balance: u64) -> Self {
        Self {
            id,
            balance: Mutex::new(balance),
        }
    }

    pub fn balance(&self) -> Result<u64, &'static str> {
        self.balance
            .lock()
            .map(|b| *b)
            .map_err(|_| "poisoned account")
    }
}

/// Both directions acquire the lower account ID first. The complete transaction
/// holds both locks: checking and changing balances are one operation.
pub fn transfer(from: &Account, to: &Account, amount: u64) -> Result<(), &'static str> {
    if from.id == to.id {
        return Err("account IDs must differ");
    }
    let (first, second) = if from.id < to.id {
        (from, to)
    } else {
        (to, from)
    };
    let mut first_balance = first.balance.lock().map_err(|_| "poisoned account")?;
    let mut second_balance = second.balance.lock().map_err(|_| "poisoned account")?;
    let (source, destination) = if from.id < to.id {
        (&mut *first_balance, &mut *second_balance)
    } else {
        (&mut *second_balance, &mut *first_balance)
    };
    if *source < amount {
        return Err("insufficient funds");
    }
    let updated = destination.checked_add(amount).ok_or("balance overflow")?;
    *source -= amount;
    *destination = updated;
    Ok(())
}

/// Force the dangerous interleaving, but use try_lock so the demo terminates.
/// Two barriers keep both first locks held until both second-lock probes finish.
pub fn conflicting_order_probe() -> usize {
    let locks = [Arc::new(Mutex::new(())), Arc::new(Mutex::new(()))];
    let barrier = Arc::new(Barrier::new(2));
    let mut handles = Vec::new();
    for index in 0..2 {
        let first = Arc::clone(&locks[index]);
        let second = Arc::clone(&locks[1 - index]);
        let rendezvous = Arc::clone(&barrier);
        handles.push(thread::spawn(move || {
            let _guard = first.lock().unwrap();
            rendezvous.wait();
            let blocked = match second.try_lock() {
                Err(TryLockError::WouldBlock) => true,
                Ok(_) => false,
                Err(TryLockError::Poisoned(_)) => panic!("probe lock poisoned"),
            };
            rendezvous.wait();
            blocked
        }));
    }
    handles
        .into_iter()
        .map(|h| usize::from(h.join().unwrap()))
        .sum()
}

#[derive(Debug)]
pub struct Summary {
    pub transfers: usize,
    pub balances: [u64; 2],
}

pub fn run_transfers(iterations: usize) -> Result<Summary, String> {
    // Each account can fund every outbound transfer without needing an inbound one.
    let initial = u64::try_from(iterations).map_err(|_| "iteration count too large")?;
    let accounts = [
        Arc::new(Account::new(1, initial)),
        Arc::new(Account::new(2, initial)),
    ];
    let start = Arc::new(Barrier::new(2));
    let mut handles = Vec::new();
    for direction in 0..2 {
        let from = Arc::clone(&accounts[direction]);
        let to = Arc::clone(&accounts[1 - direction]);
        let barrier = Arc::clone(&start);
        handles.push(thread::spawn(move || -> Result<usize, &'static str> {
            barrier.wait();
            for _ in 0..iterations {
                transfer(&from, &to, 1)?;
            }
            Ok(iterations)
        }));
    }
    let mut completed = 0;
    let mut errors = Vec::new();
    for handle in handles {
        match handle.join() {
            Ok(Ok(count)) => completed += count,
            Ok(Err(error)) => errors.push(error.to_string()),
            Err(_) => errors.push("transfer worker panicked".into()),
        }
    }
    if !errors.is_empty() {
        return Err(errors.join("; "));
    }
    let balances = [accounts[0].balance()?, accounts[1].balance()?];
    if balances != [initial, initial] {
        return Err("balance invariant violated".into());
    }
    Ok(Summary {
        transfers: completed,
        balances,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn conflicting_order_exposes_both_blocked_acquisitions() {
        assert_eq!(conflicting_order_probe(), 2);
    }

    #[test]
    fn opposite_transfers_preserve_balances_and_complete_every_operation() {
        let summary = run_transfers(10_000).unwrap();
        assert_eq!(summary.transfers, 20_000);
        assert_eq!(summary.balances, [10_000, 10_000]);
    }

    #[test]
    fn transfer_moves_money_in_both_directions() {
        let a = Account::new(1, 100);
        let b = Account::new(2, 50);
        transfer(&a, &b, 20).unwrap();
        assert_eq!((a.balance().unwrap(), b.balance().unwrap()), (80, 70));
        transfer(&b, &a, 10).unwrap();
        assert_eq!((a.balance().unwrap(), b.balance().unwrap()), (90, 60));
    }

    #[test]
    fn rejected_transfers_do_not_mutate_balances() {
        let a = Account::new(1, 10);
        let b = Account::new(2, u64::MAX);
        assert_eq!(transfer(&a, &b, 11), Err("insufficient funds"));
        assert_eq!(transfer(&a, &b, 1), Err("balance overflow"));
        assert_eq!(transfer(&a, &a, 1), Err("account IDs must differ"));
        assert_eq!((a.balance().unwrap(), b.balance().unwrap()), (10, u64::MAX));
    }

    #[test]
    fn duplicate_ids_are_rejected_and_empty_work_finishes() {
        assert!(transfer(&Account::new(1, 1), &Account::new(1, 1), 1).is_err());
        assert_eq!(run_transfers(0).unwrap().transfers, 0);
    }
}
