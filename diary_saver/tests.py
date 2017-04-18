import unittest
from unittest.mock import MagicMock
from datetime import date, time

from main import fetch_all_entries_sorted_by_date, format_diary_entry
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

if __name__ == '__main__':
    unittest.main()
