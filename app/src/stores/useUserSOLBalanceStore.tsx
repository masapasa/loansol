import { Connection, LAMPORTS_PER_SOL, PublicKey } from '@solana/web3.js';
import create, { State } from 'zustand';

interface UserSOLBalanceStore extends State {
    balanceSol: number;
    getUserSOLBalance: (publicKey: PublicKey, connection: Connection) => void;
}

const useUserSOLBalanceStore = create<UserSOLBalanceStore>((set, _get) => ({
    balanceSol: 0,
    getUserSOLBalance: async (publicKey, connection) => {
        let balanceSol = 0;
        try {
            balanceSol = await connection.getBalance(publicKey, 'confirmed');
            balanceSol = balanceSol / LAMPORTS_PER_SOL;
        } catch (e) {
            console.log(`error getting balanceSol: `, e);
        }
        set(s => {
            s.balanceSol = balanceSol;
            console.log(`balanceSol updated, `, balanceSol);
        });
    },
}));

export default useUserSOLBalanceStore;
