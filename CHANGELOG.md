# 0.1.0-alpha
- Released first version.
# 0.1.1-alpha
- Fixed issue with wrong URL in the console message.
# 0.2.0-alpha
- Added an API endpoint to drop a database.
# 0.3.0-alpha
- Switched to `VecDeque` instead of `Vec` for the database.
- Added a way to add raw bytes (`VecDeque<u8>`) to the database.
- Optimized performance by using `VecDeque::with_capacity()` instead of `VecDeque::new()`.
