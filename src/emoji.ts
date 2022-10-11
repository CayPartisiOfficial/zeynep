import { CommandInteraction } from "discord.js";

export const emoji = { name: "emoji", description: "Emoji menüsünü açar" };

export async function execute(interaction: CommandInteraction) {
	await interaction.reply("todo!");
}
