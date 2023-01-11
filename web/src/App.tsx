import Menu from "./Menu";
import Cell from "./Cell";
import Prompt from "./Prompt";
import Help from "./Help";
import Button from "./Button";
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
  max-width: 700px;
  min-width: 380px;
  margin-left: auto;
  margin-right: auto;
  box-sizing: border-box;
  padding: 1rem;
`;

const Container = styled.div`
  position: relative;
  height: 100vh;
  display:flex;
  justify-content: space-between;
`;

const Actions = styled.div`
  display: flex;
  justify-content: flex-end;
  width: 100%;
`;

const LogoContainer = styled.div`
  display: flex;
  align-items: center;
  justify-content: center;
`;

const Logo = styled.img`
    width: 6rem;
    margin-right: 0.5rem;
`;

const LogoText = styled.div`
    font-size: 1.6rem;
    font-weight: normal;
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

  return (
    <div>
      <Prompt
        title="Welcome back!"
        show={!state.sessionRestored &&
          (state.previousSession?.length ?? 0) > 0}
        action="Restore session"
        handleAction={() => recoverSession(state, dispatch)}
      >
        You have a previous session saved since last time.
      </Prompt>
      <Container>
        <div style={{ width: "100%" }}>
          <Menu helpOpen={state.helpOpen} toggleHelp={() => toggleHelp(dispatch)} />
          <LogoContainer>
            <Logo src="logo.png"></Logo>
            <LogoText>
              <strong>Hypatia</strong>
              <br />Calculator
            </LogoText>
          </LogoContainer>
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
                <Button
                  onClick={() => addCell(state, dispatch)}
                  title="Add cell"
                  icon={<MdAddCircleOutline />}
                />
              </Actions>
            </Workspace>
          )}
        </div>
        <Help show={state.helpOpen} />
      </Container>
    </div>
  );
}

export default App;
