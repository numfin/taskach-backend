export class Logger {
  constructor(private name: string) {}

  info(message: string) {
    this.writeLn(`Info: ${message}`);
  }
  error(error: Error) {
    this.writeLn(`Error: ${error.message}`);
    this.writeLn(`Stack: ${error.stack}`);
  }

  private writeLn(message: string) {
    process.stdout.write(`[${this.name}] ${message}\n`);
  }
}
