# NoteNest backend
NoteNest's backend is written entirely in Rust and utilizes the firebase_rs cargo package.
This code is used to interface with the firebase real-time database, you can do the following:
1. Create an account
2. Delete an account
3. Get information on an account
4. 
Accounts have passwords stored in this database, but are encrypted with bcrypt. 
