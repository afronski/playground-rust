all: install

install:
	virtualenv .env --python=python3.8
	.env/bin/python -m pip install -r requirements.txt

clean:
	rm -rf .env/
