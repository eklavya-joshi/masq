import { useEffect, useRef } from "react";
export default function useWebsocket(onMessage, inbox) {
    const ws = useRef(null);
    useEffect(() => {
        if (ws.current !== null) return;
        const wsUri = 'ws://localhost:8080/ws/inbox/' + inbox;
        ws.current = new WebSocket(wsUri);
        ws.current.onopen = () => console.log("ws opened");
        ws.current.onclose = () => console.log("ws closed");
        const wsCurrent = ws.current;
        // return () => {
        //     wsCurrent.close();
        // };
    }, []);
    useEffect(() => {
        if (!ws.current) return;
        ws.current.onmessage = e => {
            console.log(e);
            onMessage()
        };
    }, []);
    const sendMessage = (msg) => {
        console.log("asdf");
        if (!ws.current) return;
        ws.current.send(msg);
    }
    return sendMessage;
}