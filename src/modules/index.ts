import Logger from "../log";

import EmojiModule from "./emoji";

export const modules: Module[] = [
	/* don't forget to add new modules here */

	new EmojiModule(),
];

export interface Module {
	name: string;
	subscribedTo: string[];
	logger: Logger;
}

export function getModule(name: string): Module | undefined {
	return modules.find(m => m.name === name);
}
