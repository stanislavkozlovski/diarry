Diary Saver
---------
This little tool is meant to be used as a scheduled task on the server.
It
1. Fetches all the Diary Entries from the DB
2. Saves all of them in a .txt file
3. Sends that .txt file over e-mail to the owner
    
As such, the .env file in the /rust/ subdirectory needs to additionally have a
* SENDER_EMAIL_ADDRESS
* SENDER_EMAIL_PASSWORD
* SENDER_EMAIL_SMTP_PORT - the port of the SMTP server (e.g for gmail - 465)    
* SENDER_EMAIL_SMTP_ADDRESS - the address of the SMTP server (e.g smtp.gmail.com)

Currently works for only SSL-enabled SMTP servers