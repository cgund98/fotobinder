export interface ErrorMessage {
	code: number;
	message: string;
}

export function instanceOfErrorMessage(err: any): err is ErrorMessage {
	return 'code' in err && 'message' in err;
}
