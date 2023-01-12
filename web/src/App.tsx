import Menu from "./Menu";
import Cell from "./Cell";
import Prompt from "./Prompt";
import Help from "./Help";
import Button from "./Button";
import Logo from "./Logo";
import styled from "styled-components";
import {
  addCell,
  Cell as CellType,
  recoverSession,
  reducer,
  removeCell,
  State,
  toggleHelp,
  useWasm,
  writeCell,
} from "./state";
import { MdAddCircleOutline } from "react-icons/md";
import { useEffect, useReducer } from "react";

const Workspace = styled.div`
  box-sizing: border-box;
  padding: 1rem;
  overflow-y: auto;
  height: 100vh;
  flex-grow:0;
  flex-shrink:1;
  width: 100%;

  @media (max-width: 800px) {
    height: auto;
  }
`;

const Center = styled.div`
  max-width: 700px;
  min-width: 380px;
  margin-left: auto;
  margin-right: auto;
`

const Container = styled.div`
  position: relative;
  display:flex;
  justify-content: space-between;
  width: 100%;

  @media (max-width: 800px) {
    flex-direction: column-reverse;
    height: auto;
  }
`;

const Actions = styled.div`
  display: flex;
  justify-content: flex-end;
  width: 100%;
`;

const BigLogo = styled(Logo)`
  margin-top: 4rem;
  margin-bottom: 1rem;
`;


function App() {
  const [state, dispatch] = useReducer(
    reducer,
    {
      cells: [],
      previousSession: null,
      loaded: false,
      sessionRestored: false,
      helpOpen: false,
    },
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

  const restoreSessionPrompt = <Prompt
    title="Welcome back!"
    show={!state.sessionRestored &&
      (state.previousSession?.length ?? 0) > 0}
    action="Restore session"
    handleAction={() => recoverSession(state, dispatch)}
  >
    You have a previous session saved since last time.
  </Prompt>;

  return (
    <div>
      <Menu
        prompts={[restoreSessionPrompt]}
        helpOpen={state.helpOpen}
        toggleHelp={() => toggleHelp(dispatch)}
      />
      <Container>
        {state.loaded && (
          <Workspace>
            <Center>
              <BigLogo />
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
                <Button
                  onClick={() => addCell(state, dispatch)}
                  title="Add cell"
                  icon={<MdAddCircleOutline />}
                />
              </Actions>
            </Center>
          </Workspace>
        )}
        <Help show={state.helpOpen} />
      </Container>
    </div>
  );
}

export default App;
