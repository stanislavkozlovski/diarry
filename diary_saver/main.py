import settings
import models


def fetch_all_entries_sorted_by_date():
    """
    Returns all the Diary Entries ordered by date descending (oldest first)
    """
    return settings.session.query(models.DiaryEntry)\
        .order_by(models.DiaryEntry.creation_date.asc(), models.DiaryEntry.creation_time.asc())\
        .all()

print(fetch_all_entries_sorted_by_date())