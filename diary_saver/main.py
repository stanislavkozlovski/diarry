import settings
import models
import textwrap
import smtplib
from datetime import datetime

from email.mime.multipart import MIMEMultipart
from email.mime.application import MIMEApplication
from email.mime.text import MIMEText
from settings import (SENDER_EMAIL, SENDER_EMAIL_PASSWORD,
                      SENDER_SMTP_ADDRESS, SENDER_EMAIL_SMTP_PORT, RECEIVER_EMAIL)


def main():
    """
    Reads all the diary entries, writes them to a file and sends that file over via e-mail
    """
    while True:
        write_entries_to_file()
        try:
            send_email()
            break
        except OSError as e:
            print(f'{datetime.now()} - {e}')


def fetch_all_entries_sorted_by_date():
    """
    Returns all the Diary Entries ordered by date descending (oldest first)
    """
    return settings.session.query(models.DiaryEntry)\
        .order_by(models.DiaryEntry.creation_date.asc(), models.DiaryEntry.creation_time.asc())\
        .all()


def format_diary_entry(entry: models.DiaryEntry) -> str:
    """
    Formats a single Diary Entry to the wanted format to be outputted in the .txt file
    """
    entry_id = entry.id
    title = entry.title
    body = entry.body
    date = entry.creation_date.strftime("%d %B %Y")
    time = entry.creation_time.strftime("%H:%M")

    formatted_entry = f"""\t\t\t\t\t\t\t{title}
#{entry_id} - {date} {time}

{body}
    """
    return formatted_entry


def write_entries_to_file():
    """
    Writes all the diary entries to the file
    :return:
    """
    with open('diary.txt', 'w') as diary_file:
        for diary_entry in fetch_all_entries_sorted_by_date():
            diary_file.write(format_diary_entry(diary_entry))
            diary_file.write('\n' + '-' * 100 + '\n')


def send_email():
    """
    Sends an e-mail to the owner with the file attached
    """
    smtp_connection = smtplib.SMTP_SSL(SENDER_SMTP_ADDRESS, SENDER_EMAIL_SMTP_PORT)
    smtp_connection.ehlo()
    smtp_connection.login(SENDER_EMAIL, SENDER_EMAIL_PASSWORD)

    message = MIMEMultipart()
    message.attach(MIMEText(f'Be sure to store it in a safe place :)'))
    # Attach the file
    text_file_attachment = MIMEApplication(open("diary.txt", "rb").read())
    text_file_attachment.add_header('Content-Disposition', 'attachment', filename="diary.txt")
    message.attach(text_file_attachment)

    message['Subject'] = f'Diary saved at {datetime.now().strftime("%d %B %Y")}'
    message['From'] = SENDER_EMAIL
    message['To'] = RECEIVER_EMAIL

    smtp_connection.sendmail(SENDER_EMAIL, [RECEIVER_EMAIL], message.as_string())

    smtp_connection.quit()


if __name__ == '__main__':
    main()
