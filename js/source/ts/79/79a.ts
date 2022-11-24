const dirs = [[0,1], [0,-1], [1,0], [-1,0]];

function exist(board: string[][], word: string): boolean {
    const check = (y: number, x: number, mem: string, lBoard: string[][]): boolean => {
        if(board[y][x] !== word[0]){
            return false;
        }

        const qPos = [[y,x]];
        const qMem = [mem]
        const qBoard = [lBoard];

        while (qPos.length > 0) {
            const [y,x] = qPos.pop() as number[];
            const mem = qMem.pop() as string;
            const board = qBoard.pop() as Array<Array<string>>;

            if(board[y][x] === word[mem.length]){
                board[y][x] = "-";
                if(mem + word[mem.length] === word){
                    return true;
                }
            }

            for(const [dy, dx] of dirs){
                const ny = y+dy;
                const nx = x+dx;
                if(ny>=0 && ny<board.length && nx>=0 && nx<board[0].length){
                    if(board[ny][nx] !== "-"){
                        if(board[ny][nx] === word[mem.length+1]){
                            qPos.push([ny,nx])
                            qMem.push(mem + word[mem.length])
                            qBoard.push([...board.map((row) => [...row])])
                        }
                    }
                }
            }
        }

        return false;
    }

    for (let y = 0; y<board.length; y++){
        for(let x = 0; x<board[0].length; x++){
            if(check(y, x, "", [...board.map((row) => [...row])])){
                return true;
            }
        }
    }

    return false;
};

export { exist };
