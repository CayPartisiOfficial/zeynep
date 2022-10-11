/* eslint-disable @typescript-eslint/no-non-null-assertion */
// TODO(admi): check if null & error

import * as dotenv from "dotenv";
dotenv.config();

export const TOKEN: string = process.env["TOKEN"]!;
export const TEST: boolean = process.env["TEST"] == "1";
export const TEST_GUILD: string = process.env["TEST_GUILD"]!;
