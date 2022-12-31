import init, {
  clear_state,
  insert_cell,
  read_cell_code,
  read_cell_output,
  read_cell_time,
  remove_cell,
  write_cell,
} from "web_bindings";
import { useEffect } from "react";

export type Cell = {
  code: string;
  output: string;
  time?: string;
};

export type State = {
  cells: Cell[];
  helpOpen: boolean;
  previousSession: string[] | null;
  sessionRestored: boolean;
  loaded: boolean;
};

export type Action =
  | {
    type: "write";
    updatedCells: Uint32Array;
    writtenCell: number;
  }
  | {
    type: "loaded wasm";
  }
  | {
    type: "add cell";
  }
  | {
    type: "remove cell";
    index: number;
  }
  | {
    type: "restore session";
  }
  | {
    type: "toggle help";
  };

// Note: we don't wan't to sync the web assembly state inside of the reducer since it is pure function.
// Instead, make use of the utility function that both dispatches an event and also updates the state
export const reducer = (state: State, action: Action): State => {
  switch (action.type) {
    case "write": {
      let cells = [...state.cells];
      cells[action.writtenCell].code = read_cell_code(action.writtenCell);

      // Read the output of all dependent cells (including the one we wrote to)
      action.updatedCells.forEach((index) => {
        cells[index].output = read_cell_output(index);
        cells[index].time = read_cell_time(index);
      });

      return { ...state, cells };
    }

    case "loaded wasm": {
      return { ...state, cells: [{ code: "", output: "" }], loaded: true };
    }

    case "add cell": {
      let cells = [...state.cells];
      cells.push({
        code: "",
        output: "",
      });
      return { ...state, cells };
    }

    case "remove cell": {
      return {
        ...state,
        cells: state.cells.filter((_c, i) => i !== action.index),
      };
    }

    case "restore session": {
      if (state.previousSession === null) return state;

      return {
        ...state,
        sessionRestored: true,
        cells: state.previousSession.map((code, i) => ({
          code: read_cell_code(i),
          output: read_cell_output(i),
          time: read_cell_time(i),
        })),
      };
    }

    case "toggle help": {
      return {
        ...state,
        helpOpen: !state.helpOpen,
      };
    }
  }
};

export function toggleHelp(dispatch: (action: Action) => void) {
  dispatch({ type: "toggle help" });
}

export function writeCell(
  index: number,
  code: string,
  dispatch: (action: Action) => void,
) {
  const updatedCells = write_cell(index, code);
  dispatch({ type: "write", updatedCells, writtenCell: index });
}

export function addCell(
  state: State,
  dispatch: (action: Action) => void,
) {
  insert_cell(state.cells.length);
  dispatch({ type: "add cell" });
}

export function removeCell(index: number, dispatch: (action: Action) => void) {
  remove_cell(index);
  dispatch({ type: "remove cell", index });
}

export function recoverSession(
  state: State,
  dispatch: (action: Action) => void,
) {
  clear_state();
  if (state.previousSession === null) return;

  state.previousSession.forEach((code, index) => {
    insert_cell(index);
    write_cell(index, code);
  });

  dispatch({ type: "restore session" });
}

export function useWasm(dispatch: (action: Action) => void) {
  useEffect(() => {
    init().then(() => {
      insert_cell(0);
      dispatch({ type: "loaded wasm" });
    });
  }, []);
}
