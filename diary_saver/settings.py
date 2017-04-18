# settings.py
import os
from os.path import join, dirname
from dotenv import load_dotenv
import socket
socket.setdefaulttimeout(120)  # set a timeout for the email connection

from sqlalchemy import create_engine
from sqlalchemy.ext.declarative import declarative_base
from sqlalchemy.orm import sessionmaker

os.chdir('../rust')  # navigate to rust's folder path
rust_folder_path = os.getcwd()
os.chdir('../')  # navigate back to the root folder of the app, so that diary.txt is saved there
dotenv_path = join(rust_folder_path, '.env')
load_dotenv(dotenv_path)

DATABASE_URL = os.environ.get("DATABASE_URL")
RECEIVER_EMAIL = os.environ.get("EMAIL")
SENDER_EMAIL = os.environ.get("SENDER_EMAIL_ADDRESS")
SENDER_EMAIL_PASSWORD = os.environ.get("SENDER_EMAIL_PASSWORD")
SENDER_EMAIL_SMTP_PORT = os.environ.get("SENDER_EMAIL_SMTP_PORT")
SENDER_SMTP_ADDRESS = os.environ.get("SENDER_SMTP_ADDRESS")

if not all([DATABASE_URL, RECEIVER_EMAIL, SENDER_EMAIL, SENDER_EMAIL_SMTP_PORT,
            SENDER_EMAIL_PASSWORD, SENDER_SMTP_ADDRESS]):
    raise Exception('Environment variables are not properly set!')


# Setup SQLAlchemy
engine = create_engine(DATABASE_URL)
session = sessionmaker(bind=engine)()
Base = declarative_base()