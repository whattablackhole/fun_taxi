from bot_tasks import run_bot

def start_bots():
    for i in range(7):
        run_bot.delay(i)

if __name__ == "__main__":
    start_bots()

