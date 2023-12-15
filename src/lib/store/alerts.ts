import { v4 as uuidv4 } from 'uuid';
import { writable } from 'svelte/store';

import { instanceOfErrorMessage } from '../api/error';

export enum Severity {
	Good = 'good',
	Warn = 'warn',
	Bad = 'bad'
}

export interface Alert {
	id: string;
	message: string;
	severity: Severity;
}
export const alerts = writable<Alert[]>([]);

// Define helper methods to add alerts
export const good = (message: string) => {
	alerts.update((a) => {
		a.push({
			id: uuidv4(),
			message,
			severity: Severity.Good
		});
		return a;
	});
};

export const warn = (message: string) => {
	alerts.update((a) => {
		a.push({
			id: uuidv4(),
			message,
			severity: Severity.Warn
		});
		return a;
	});
};

export const bad = (message: string) => {
	alerts.update((a) => {
		a.push({
			id: uuidv4(),
			message,
			severity: Severity.Bad
		});
		return a;
	});
};

export const catchBad = (err: any) => {
	console.warn(err);
	let message = JSON.stringify(err);
	if (instanceOfErrorMessage(err)) {
		message = err.message;

		// Bad request has a reason substring
		if (err.code == 400) message = err.message.split("'")[1];
	}

	bad(message);
};
