configure:
	pre-commit install

precommit:
	pre-commit run --all-files
