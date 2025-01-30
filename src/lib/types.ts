export type ProcessEntry = {
    name: string;
    topic: string;
    value: string;
    off_value: string;

};

export type ProcessEntryWithIndex = {
    index: number;
    name: string;
    topic: string;
    value: string;
    offValue: string;
    deleteEntry: () => void;
};