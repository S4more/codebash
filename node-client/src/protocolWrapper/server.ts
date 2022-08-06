import { Socket } from "./socket";

export class Server {

    constructor(private readonly socket: Socket, readonly userId: string) {
    }

    public sendChatMessage(message: string)  {
        this.socket.sendMessage({
            'ChatMessage' : {
                message,
                user_id: this.userId
            }
        });
    }
}