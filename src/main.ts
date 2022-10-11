import * as env from "./env";

import log from "npmlog";
import { Client } from "discord.js";

import { modules } from "./modules";

log.level = env.TEST ? "verbose" : "info";
log.stream = process.stdout;

export const client = new Client({
	intents: [],
});

function stop() {
	log.info("", "stopping");

	modules.forEach(module => {
		if (module.subscribedTo.includes("quit")) {
			module.logger.info("stopping");
			// eslint-disable-next-line @typescript-eslint/ban-ts-comment
			// @ts-ignore
			module.quit();
		}
	});

	client.destroy();
	process.exit();
}

process.once("SIGINT", stop);
process.once("SIGTERM", stop);

modules.forEach(module => {
	module.subscribedTo.forEach(event => {
		module.logger.debug("registering handler", { event });

		// this is handled somewhere else
		if (event === "quit") {
			return;
		}

		client.on(event, async (...args) => {
			module.logger.debug("dispatching", { event });

			try {
				// eslint-disable-next-line @typescript-eslint/ban-ts-comment
				// @ts-ignore
				await module[event](...args);
			} catch (e) {
				module.logger.error(`Error while dispatching`, e as Error, {
					event,
				});
			}
		});
	});
});

client.on("ready", () => {
	// eslint-disable-next-line @typescript-eslint/no-non-null-assertion
	log.info("", `Logged in as ${client.user!.tag}`);
});

log.info("", "Trying to log in");
client.login(env.TOKEN);
