import Menu from "./Menu";
import Cell from "./Cell";
import Prompt from "./Prompt";
import styled from "styled-components";
import {
  addCell,
  Cell as CellType,
  recoverSession,
  reducer,
  removeCell,
  State,
  useWasm,
  writeCell,
} from "./state";
import { MdAddCircleOutline } from "react-icons/md";
import { useEffect, useReducer } from "react";

const Workspace = styled.div`
  width: 100%;
  max-width: 700px;
  margin-left: auto;
  margin-right: auto;
  box-sizing: border-box;
  padding: 1rem;
`;

const Actions = styled.div`
  display: flex;
  max-width: 150px;
  width: 100%;
`;

const Action = styled.button`
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-sizing: border-box;
  padding: 0.5rem;
  width: 100%;
  margin-top: 0.5rem;
  margin-right: 0.5rem;
  border: none;
  border-radius: 0.2rem;
  background: #F0C808;
  border: solid 2px black;
`;

function App() {
  const [state, dispatch] = useReducer(
    reducer,
    { cells: [], previousSession: null, loaded: false, sessionRestored: false },
    (state: State) => {
      // Load the previous session from local
      let parsedData = localStorage.getItem("cells");
      if (!parsedData) {
        return { ...state, previousSession: [] };
      }
      const previousSession: Array<string> = JSON.parse(parsedData);

      if (previousSession.length === 0) {
        return { ...state, previousSession: [] };
      }

      if (previousSession.every((code) => code.length === 0)) {
        return { ...state, previousSession: [] };
      }

      return { ...state, previousSession };
    },
  );

  // load the wasm code
  useWasm(dispatch);

  // Keep local storage in sync with the current state so that we
  // can recover it the next time the program is opened.
  useEffect(() => {
    if (state.previousSession === null) return;
    localStorage.setItem(
      "cells",
      JSON.stringify(state.cells.map((cell) => cell.code)),
    );
  }, [state.cells, state.previousSession]);

  return (
    <div className="App">
      <Prompt
        title="Welcome back!"
        show={!state.sessionRestored &&
          (state.previousSession?.length ?? 0) > 0}
        action="Restore session"
        handleAction={() => recoverSession(state, dispatch)}
      >
        You have a previous session saved since last time.
      </Prompt>
      <Menu />
      {state.loaded && (
        <Workspace>
          {state.cells.map((cell, index) => (
            <Cell
              key={index}
              noAnimation={index == 0}
              code={cell.code}
              output={cell.output}
              time={cell.time}
              onChange={(i, code) => writeCell(i, code, dispatch)}
              onRemove={(i) => removeCell(i, dispatch)}
              addCellAction={() => addCell(state, dispatch)}
              index={index}
            />
          ))}
          <Actions>
            <Action onClick={() => addCell(state, dispatch)}>
              New Cell <MdAddCircleOutline size="1.2rem" />
            </Action>
          </Actions>
        </Workspace>
      )}
    </div>
  );
}

export default App;
