import log from "npmlog";

export default class Logger {
	private name: string;

	constructor(name: string) {
		this.name = name;
	}

	debug(message: string, fields: { [key: string]: unknown }): void {
		log.verbose(this.name, message, fields);
	}

	info(message: string, fields?: { [key: string]: unknown }): void {
		log.info(this.name, message, fields);
	}

	warn(message: string, fields?: { [key: string]: unknown }): void {
		log.warn(this.name, message, fields);
	}

	error(message: string, error?: Error, fields?: { [key: string]: unknown }) {
		log.error(this.name, message, [fields, error]);
	}
}
