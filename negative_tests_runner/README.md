# negative_tests_runner

This runs negative (failing) tests/examples in [../negative_tests/](../negative_tests/), and it
verifies the output errors.

We tried `trybuild` crate, but it didn't work here. So using `snapbox`.
