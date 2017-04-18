import unittest
from unittest.mock import MagicMock, patch
from datetime import date, time

from main import fetch_all_entries_sorted_by_date, format_diary_entry, send_email
from models import DiaryEntry


class Tests(unittest.TestCase):
    def test_fetch_all_entries_sorted_by_date(self):
        """ Should return the Diary Entries sorted by date """
        diary_entries: [DiaryEntry] = fetch_all_entries_sorted_by_date()
        for idx in range(1, len(diary_entries)):
            prev_entry = diary_entries[idx]
            curr_entry = diary_entries[idx]
            if prev_entry.creation_date == curr_entry.creation_date:
                self.assertTrue(prev_entry.creation_time <= curr_entry.creation_time)
            else:
                self.assertTrue(prev_entry.creation_date < curr_entry.creation_time)

    def test_format_diary_entry_formats_correctly(self):
        diary_entry = MagicMock(title="Train", body="Training Insane", id=1,
                                creation_date=date(month=4, day=4, year=2017),
                                creation_time=time(hour=14, minute=50))
        expected_output = """							Train
#1 - 04 April 2017 14:50

Training Insane"""

        self.assertIn(expected_output, format_diary_entry(diary_entry))

    @patch('main.smtplib.SMTP_SSL')
    def test_send_email(self, SMTP_SSL_mock):
        from settings import SENDER_SMTP_ADDRESS, SENDER_EMAIL_SMTP_PORT, SENDER_EMAIL, SENDER_EMAIL_PASSWORD
        login_mock = MagicMock()
        send_email_mock = MagicMock()
        smtp_connection_mock = MagicMock(login=login_mock, sendmail=send_email_mock)
        SMTP_SSL_mock.return_value = smtp_connection_mock

        send_email()

        # Assert it tries to establish a connection
        SMTP_SSL_mock.assert_called_once_with(SENDER_SMTP_ADDRESS, SENDER_EMAIL_SMTP_PORT)
        # assert it tries to log in
        login_mock.assert_called_once_with(SENDER_EMAIL, SENDER_EMAIL_PASSWORD)
        # assert it tries to send the email
        send_email_mock.assert_called_once()


if __name__ == '__main__':
    unittest.main()
