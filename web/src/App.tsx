import Menu from "./Menu";
import Cell from "./Cell";
import Prompt from "./Prompt";
import styled from "styled-components";
import init, {read_cell, write_cell, insert_cell, remove_cell, clear_state, read_cell_time} from 'web_bindings';
import {MdAddCircleOutline} from "react-icons/md";
import {useEffect, useState} from "react";

const Workspace = styled.div`
  width: 100%;
  max-width: 700px;
  margin-left:auto;
  margin-right:auto;
  box-sizing: border-box;
  padding: 1rem;
`;


const Actions = styled.div`
  display: flex;
  max-width: 150px;
  width: 100%;
`

const Action = styled.button`
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: space-between;
  box-sizing: border-box;
  padding: 0.5rem;
  width:100%;
  margin-top: 0.5rem;
  margin-right: 0.5rem;
  border: none;
  border-radius: 0.2rem;
`;

type Cell = {
    code: string,
    output: string,
    time?: string,
};

function App() {
    const [loaded, setLoaded] = useState(false);
    const [prevSession, setPreviousSession] = useState<Array<string>>([]);
    const [cells, setCells] = useState<Array<Cell>>([{
        code: '',
        output: '',
    }]);

    // Load the WASM file
    useEffect(() => {
        init().then(() => {
            setLoaded(true);
            insert_cell(0);
        });
    }, []);

    // Check if there is content in local storage from a previous session
    useEffect(() => {
        let parsedData = localStorage.getItem('cells');
        if (!parsedData) {
            return;
        }

        const prevCells: Array<string> = JSON.parse(parsedData);

        if (prevCells.length === 0) {
            return;
        }

        if (prevCells.every(code => code.length === 0)) {
            return;
        }

        setPreviousSession(prevCells);
    }, []);

    // Keep local storage in sync with the current state
    useEffect(() => {
        localStorage.setItem('cells', JSON.stringify(cells.map(cell => cell.code)));
    }, [cells]);

    const onChange = (changed_cell_index: number, code: string) => {
        if (!loaded) return;
        const updatedCells = write_cell(changed_cell_index, code);
        // update the state to reflect the changes
        setCells(oldCells => {
            let cells = [...oldCells];
            cells[changed_cell_index].code = code;
            updatedCells.forEach((index: number) => {
                cells[index].output = read_cell(index);
                cells[index].time = read_cell_time(index);
            });
            return cells;
        });
    }

    const addCell = () => {
        insert_cell(cells.length);

        setCells(oldCells => {
            let cells = [...oldCells];
            cells.push({
                code: '',
                output: '',
            });
            return cells;
        });
    }

    const removeCell = (index: number) => {
        remove_cell(index);

        setCells(oldCells => {
            let cells = [...oldCells];
            cells.splice(index, 1);
            return cells;
        });
    }

    // restore the previous session
    const restoreSession = () => {
        clear_state();
        prevSession.forEach((code, index) => {
            insert_cell(index);
            write_cell(index, code)
        });

        setCells(prevSession.map(code => ({code: code, output: ''})));

        setPreviousSession([]);
    };

    return <div className="App">
        <Menu/>
        {loaded && <Workspace>
            <Prompt
                title="Welcome back!"
                show={prevSession.length > 0}
                action="Restore session"
                handleAction={restoreSession}
            >
                You have a previous session saved since last time.
            </Prompt>
            {cells.map((cell, index) => <Cell
                key={index}
                noAnimation={index == 0}
                code={cell.code}
                output={cell.output}
                time={cell.time}
                onChange={onChange}
                onRemove={removeCell}
                addCellAction={addCell}
                index={index}
            />)}
            <Actions>
                <Action onClick={addCell}>
                    New Cell <MdAddCircleOutline size="1.2rem"/>
                </Action>
            </Actions>

        </Workspace>}
    </div>
}

export default App
