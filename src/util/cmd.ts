import { ApplicationCommandData, Snowflake } from "discord.js";
import log from "npmlog";
import { client } from "../main";
import * as env from "../env";

export async function registerSlashCommand(
	cmd: ApplicationCommandData
): Promise<void> {
	if (env.TEST) {
		log.verbose(
			"",
			"Test Mode! Registering slash command only for test guild!"
		);

		await (
			await client.guilds.fetch(env.TEST_GUILD as Snowflake)
		).commands.create(cmd);
	} else {
		await client.application?.commands.create(cmd);
	}
}
