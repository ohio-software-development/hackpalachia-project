const accountSid = 'ACa59335fa148b49926bbbd3943f1c3cd5';
const authToken = '00eea9256dbadd747524ee22d71b8c81';
const client = require('twilio')(accountSid, authToken);

client.messages
    .create({
        body: 'Hello, thanks for using our application!  This is where we will send out important information (health care availability updates and such).  At any time, send "help" and we will connect you with staff who care about you here at Ohio University.',
        from: '+18442431995',
        to: '+16144411191'
    })
    .then(message => console.log(message.sid)).done();