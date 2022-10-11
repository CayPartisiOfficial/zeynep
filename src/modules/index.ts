import Logger from "../log";

export const modules: Module[] = [
	/* don't forget to add new modules here */
];

export interface Module {
	name: string;
	subscribedTo: string[];
	logger: Logger;
}

export function getModule(name: string): Module | undefined {
	return modules.find(m => m.name === name);
}
