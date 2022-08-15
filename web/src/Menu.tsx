import styled from 'styled-components';

const Container = styled.div`
    display: flex;
    width: 100%;
    box-sizing: border-box;
    padding: 1rem;
    justify-content: space-between;
`

const Logo = styled.div`
    font-size: 2rem;
    font-weight: bold;
`

const Button = styled.button`
`

export default function Menu() {
    return <Container className="Menu">
        <Logo>Hypatia</Logo>
        <div>
            <Button>Reference</Button>
            <Button>Github</Button>
        </div>
    </Container>
}