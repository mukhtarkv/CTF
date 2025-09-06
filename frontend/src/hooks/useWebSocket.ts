"use client";

import { useEffect, useRef, useState } from "react";

interface WebSocketMessage {
  type: string;
  [key: string]: any;
}

interface UseWebSocketProps {
  url: string;
  onMessage?: (message: WebSocketMessage) => void;
  onOpen?: () => void;
  onClose?: () => void;
  onError?: (error: Event) => void;
}

export const useWebSocket = ({ url, onMessage, onOpen, onClose, onError }: UseWebSocketProps) => {
  const [isConnected, setIsConnected] = useState(false);
  const [connectionState, setConnectionState] = useState<'connecting' | 'connected' | 'disconnected' | 'error'>('disconnected');
  const wsRef = useRef<WebSocket | null>(null);
  const urlRef = useRef<string>('');

  const sendMessage = (message: any) => {
    if (wsRef.current && wsRef.current.readyState === WebSocket.OPEN) {
      wsRef.current.send(JSON.stringify(message));
    }
  };

  const connect = () => {
    // Prevent duplicate connections
    if (!url || urlRef.current === url) {
      return;
    }
    
    if (wsRef.current?.readyState === WebSocket.OPEN) {
      return;
    }

    console.log('Connecting to WebSocket:', url);
    setConnectionState('connecting');
    wsRef.current = new WebSocket(url);
    urlRef.current = url;

    wsRef.current.onopen = () => {
      setIsConnected(true);
      setConnectionState('connected');
      onOpen?.();
    };

    wsRef.current.onmessage = (event) => {
      try {
        const message = JSON.parse(event.data);
        onMessage?.(message);
      } catch (error) {
        console.error('Failed to parse WebSocket message:', error);
      }
    };

    wsRef.current.onclose = () => {
      setIsConnected(false);
      setConnectionState('disconnected');
      onClose?.();
    };

    wsRef.current.onerror = (error) => {
      setConnectionState('error');
      onError?.(error);
    };
  };

  const disconnect = () => {
    if (wsRef.current) {
      wsRef.current.close();
      wsRef.current = null;
      urlRef.current = '';
    }
  };

  useEffect(() => {
    return () => {
      disconnect();
    };
  }, []);

  return {
    isConnected,
    connectionState,
    sendMessage,
    connect,
    disconnect,
  };
};