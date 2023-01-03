# email-notif-rs

Email notification of process status. For long-running processes (e.g. training a ML model or running a simulation) it can be useful to be sent a notification on completion. Email is a handy way to recieve this.

This is a rust implementation of my python script [`email_notifier`](github.com/cbosoft/email_notifier).

# Installation

This package is not on cargo (yet) but will be eventually. For now, add the dependency to your projects as:

```toml
[dependency]
email-notif-rs = { git = "https://github.com/cbosoft/email-notif-rs" }
```

# Usage

You need to tell the library where emails should be sent from. I don't want to put email config in programs, so it's in a config file in your home dir. The sending email should be one you don't mind having a password in plain text in a json file. (i.e. one that was set up for the purposes of sending notifications.)

```json
{
  "smtp_server": "smtp.example.com",
  "sender_email": "notifications@bar.com",
  "password": "notVerySecureAtAll",
  "recipient_email": "foo@bar.com",
  "port": 587
}
```

With the config set, you can now use the library from a program as follows:


```rust
use email_notif::EmailNotifier;
use crate::foo::long_running_process;

fn main() {
  EmailNotifier::new("Simulation Run")
    .capture(
      |em| {
        for i in 0..10 {
          long_running_process();
          em.send_update(format!("iteration {i} complete"));
        }
      }
    );
}
```

The above will send an update notification every time an iteration is complete, and an email when the process completes successfully. In addition, an email will be sent if the process results in panic. (Only an unwind panic, however.)
