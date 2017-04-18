import unittest
from main import fetch_all_entries_sorted_by_date
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


if __name__ == '__main__':
    unittest.main()