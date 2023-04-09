import { writable, derived, type Readable } from 'svelte/store';

const TIMEOUT = 3000;

type ToastType = 'default' | 'danger' | 'warning' | 'info' | 'success';
type Toast = {
	id: string;
	type: ToastType;
	message: string;
	timeout: number;
};

function createNotificationStore(timeout = TIMEOUT) {
	const _notifications = writable<Toast[]>([]);

	function send(message: string, type: ToastType = 'default', timeout = TIMEOUT) {
		const newid = id();
		_notifications.update((state: Toast[]) => {
			return [...state, { id: newid, type, message, timeout }];
		});
		setTimeout(() => {
			_notifications.update(($_notifications) => $_notifications.filter((n) => n.id !== newid));
		}, timeout);
	}

	const notifications = derived(_notifications, ($_notifications, set) => {
		set($_notifications);
	}) satisfies Readable<Toast[]>;
	const { subscribe } = notifications;

	return {
		subscribe,
		send,
		default: (msg: string, timeout: number) => send(msg, 'default', timeout),
		danger: (msg: string, timeout: number) => send(msg, 'danger', timeout),
		warning: (msg: string, timeout: number) => send(msg, 'warning', timeout),
		info: (msg: string, timeout: number) => send(msg, 'info', timeout),
		success: (msg: string, timeout: number) => send(msg, 'success', timeout)
	};
}

function id() {
	return '_' + Math.random().toString(36).substr(2, 9);
}

export const notifications = createNotificationStore();
