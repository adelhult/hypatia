import styled from "styled-components";
const ButtonEl = styled.button<{ active: boolean }>`
  border: none;
  border-radius: 0.3rem;
  padding: 0.5rem;
  display: flex;
  align-items: center;
  gap: 0.3rem;
  font-size: 1rem;
  background: ${props => props.active ? "#bcc0c3" : "dbdfe2"};
  cursor: pointer;
  transition: background-color 0.2s;

  &:hover {
    background: #bcc0c3;
}
`;

type ButtonProps = {
    title: string;
    icon?: JSX.Element
    active?: boolean;
    onClick: () => void,
}

export default function Button({ title, icon, onClick, active }: ButtonProps) {
    return <ButtonEl active={active ?? false} onClick={onClick}>
        {title} {icon ?? ""}
    </ButtonEl>
}
