import styled from 'styled-components';
import Button from './Button';

const Container = styled.div`
    display: flex;
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
    justify-content: space-between;
    align-items: center;
    margin-bottom: 1rem;
`

const Logo = styled.div`
    font-size: 1.6rem;
    font-weight: bold;
`

export default function Menu() {
    return <Container className="Menu">
        <Logo>Hypatia</Logo>
        <div>
            <Button>Github</Button>
        </div>
    </Container>
}