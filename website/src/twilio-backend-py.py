from twilio.rest import Client
from flask import Flask, render_template
app = Flask(__name__)

@app.route('/my-link/')
def my_link():
    account_sid = 'ACa59335fa148b49926bbbd3943f1c3cd5'
    auth_token = '00eea9256dbadd747524ee22d71b8c81'
    client = Client(account_sid, auth_token)

    message = client.messages.create(
        from_='+18442431995',
        body='Hello, thanks for using our application!  This is where we will send out important information (health care availability updates and such).  At any time, send "help" and we will connect you with staff who care about you here at Ohio University.',
        to='+16144411191'
    )

    print(message.sid)
if __name__ == '__main__':
  app.run(debug=True)