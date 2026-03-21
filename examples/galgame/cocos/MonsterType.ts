/**
 * MonsterType接口
 */
export interface MonsterType {
    /**
     * MonsterTypeID
     */
    id: number;
    /**
     * MonsterType名称
     */
    name: string;
    /**
     * MonsterType描述
     */
    description: string;
}

/**
 * MonsterType枚举
 */
export const MonsterType = {
    /**
     * 普通MonsterType
     */
    NORMAL: {
        id: 1,
        name: "普通",
        description: "普通MonsterType"
    },
    /**
     * 不死MonsterType
     */
    UNDEAD: {
        id: 2,
        name: "undead",
        description: "undead MonsterType"
    },
    /**
     * 野兽MonsterType
     */
    BEAST: {
        id: 3,
        name: "野兽",
        description: "野兽MonsterType"
    },
    /**
     * 人形生物MonsterType
     */
    HUMANOID: {
        id: 4,
        name: "人形",
        description: "人形生物MonsterType"
    },
    /**
     * 巨型生物MonsterType
     */
    GIANT: {
        id: 5,
        name: "巨型",
        description: "巨型生物MonsterType"
    },
    /**
     * 龙MonsterType
     */
    DRAGON: {
        id: 6,
        name: "龙",
        description: "龙MonsterType"
    },
    /**
     * 元素生物MonsterType
     */
    ELEMENTAL: {
        id: 7,
        name: "元素",
        description: "元素生物MonsterType"
    }
} as const as Record<string, MonsterType>;
