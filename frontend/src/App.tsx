import { IMessageEvent, w3cwebsocket } from "websocket";
import { useState, useEffect, useRef } from "react";
import { ThemeProvider, createTheme } from "@mui/material/styles";
import { Box, Zoom, Fab, TextField } from "@mui/material";
import ClearIcon from "@mui/icons-material/Clear";
import CopyIcon from "@mui/icons-material/FileCopy";

type ActionType = {
  type: "get" | "update" | "clear";
  data?: string;
};

const darkTheme = createTheme({
  palette: {
    primary: {
      main: "#2196f3"
    },
    secondary: {
      main: "#3d5afe",
    },
    mode: "dark",
  },
});

function App() {
  const websocket = useRef<w3cwebsocket | null>(null);
  const [message, setMessage] = useState<string>("");
  const dispatch = (action: ActionType) =>
    websocket.current?.send(JSON.stringify(action));
  const update = ({ target: { value = "" } }) =>
    dispatch({ type: "update", data: value });

  useEffect(() => {
    websocket.current = new w3cwebsocket("ws://localhost:8000/ws");
    websocket.current.onopen = () => dispatch({ type: "get" });
    websocket.current.onmessage = (message: IMessageEvent) => {
      setMessage(message.data.toString());
    };
    return () => websocket.current?.close();
  }, []);

  return (
    <ThemeProvider theme={darkTheme}>
      <Box
        sx={{
          bgcolor: "background.paper",
          height: "100dvh",
          width: "100dvw",
          display: "flex",
          alignContent: "center",
          alignItems: "center",
        }}
      >
        <TextField
          label="Online Clipboard"
          id="main-text-area"
          value={message}
          onChange={update}
          multiline
          maxRows={10}
          sx={{
            margin: "1em",
            width: "100%",
          }}
        />
        <Zoom in={true} timeout={500}>
          <Fab
            color="secondary"
            aria-label="copy"
            style={{
              position: "absolute",
              bottom: 16 + 56 + 16,
              right: 16,
            }}
            onClick={() => navigator?.clipboard?.writeText?.(message)}
          >
            <CopyIcon />
          </Fab>
        </Zoom>
        <Zoom in={true} timeout={350}>
          <Fab
            color="secondary"
            aria-label="clear"
            style={{
              position: "absolute",
              bottom: 16,
              right: 16,
            }}
            onClick={() => dispatch({ type: "clear" })}
          >
            <ClearIcon />
          </Fab>
        </Zoom>
      </Box>
    </ThemeProvider>
  );
}

export default App;
