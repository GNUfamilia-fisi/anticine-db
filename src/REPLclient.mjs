import net from 'node:net'
import readline from 'node:readline'

const input = readline.createInterface({
  input: process.stdin,
  output: process.stdout,
  terminal: false
});

const client = new net.Socket();

const DATABASE_PORT = 7878;

const OPTIONS = {
  host: '127.0.0.1',
  port: DATABASE_PORT,
};

client.connect(OPTIONS);

input.on('line', (line) => {
  if (!client.write(`${line}\n`)) {
    console.log('failed to write to socket');
  }
});

client.on('data', (data) => {
  console.log(`received: ${data.toString()}`);
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
