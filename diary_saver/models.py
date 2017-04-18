from settings import Base
from sqlalchemy import Column, Integer, String, Date, Time


class DiaryEntry(Base):
    __tablename__ = 'diary_entries'

    id = Column(Integer, primary_key=True)
    title = Column(String)
    body = Column(String)
    creation_date = Column(Date)
    creation_time = Column(Time)

    def __repr__(self):
        return f'Diary Entry #{self.id} - {self.title}. Created on {self.creation_date}{self.creation_time}'
