class Token:
    def __init__(self, value, type, status, line):
        self.value = value
        self.type = type
        self.status = status
        self.line = line