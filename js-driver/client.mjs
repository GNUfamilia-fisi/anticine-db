import net from 'node:net'
import readline from 'node:readline'

const input = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

const client = new net.Socket();

const DATABASE_PORT = 7868;

const OPTIONS = {
  host: '127.0.0.1',
  port: DATABASE_PORT,
};

const SESSION = {
  id: 0,
  columns: [{
    name: 'a',
    num: '1',
    seats: [
      {
        id: 0,
        name: 'id'
      },
      {
        id: 1,
        name: 'some\n\ndata\ntest'
      }
    ]
  }]
}

const TO_WRITE = `SET sessions.123123 ${JSON.stringify(SESSION)}`;

client.connect(OPTIONS, () => {
  console.log(`\nConectado existosamente a la Anticine-db\n`);
});

input.on('line', (line) => {
  if (line === 'test') {
    client.write(TO_WRITE);
    console.log(`sent: ${TO_WRITE}`);
    return;
  }

  if (!client.write(`${line}`)) {
    console.log('failed to write to socket');
  }
});

client.on('data', (data) => {
  console.log(`-> ${data.toString()}\n`);
});

client.on('close', (data) => {
  console.log(`connection closed`, data);
});

client.on('error', (e) => {
  if (e.code === 'ECONNREFUSED') {
    console.error(`Can't connect to ${OPTIONS.host}:${OPTIONS.port}`);
    client.destroy();
    process.exit(1);
  }
});
