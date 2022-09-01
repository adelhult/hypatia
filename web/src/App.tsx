import Menu from "./Menu";
import Cell from "./Cell";
import styled from "styled-components";
import init, { read_cell, write_cell, insert_cell } from 'web_bindings';
import { MdAddCircleOutline, MdFormatAlignLeft } from "react-icons/md";
import { useEffect, useState } from "react";

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
  border-radius: 5px;
`;

type Cell = {
  code: string,
  output: string,
};

function App() {
  const [loaded, setLoaded] = useState(false);
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

  const onChange = (changed_cell_index: number, code: string) => {
    if (!loaded) return;
    const updatedCells = write_cell(changed_cell_index, code);
    // update the state to reflect the changes
    setCells(oldCells => {
      let cells = [...oldCells];
      cells[changed_cell_index].code = code;
      updatedCells.forEach((index: number) => {
        cells[index].output = read_cell(index);
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

  return <div className="App">
      <Menu />
      {loaded && <Workspace>
        {cells.map((cell, index) => <Cell
          key={index}
          code={cell.code}
          output={cell.output}
          onChange={onChange}
          addCellAction={addCell}
          index={index}
        />)}
        <Actions>
          <Action onClick={addCell}>
            New Cell <MdAddCircleOutline size="1.2rem" />
          </Action>
        </Actions>

      </Workspace>}
    </div>
}

export default App
