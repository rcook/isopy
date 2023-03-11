.DEFAULT_GOAL := default

default:
	pyinstaller --onefile -n isopy isopy_bin/main.py
