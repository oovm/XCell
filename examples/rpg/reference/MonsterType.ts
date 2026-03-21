

/**
 * 怪物类型接口
 */
export interface MonsterType {
    /**
     * 怪物类型ID
     */
    id: number;
    /**
     * 怪物类型名称
     */
    name: string;
    /**
     * 怪物类型描述
     */
    description: string;
}

/**
 * 怪物类型枚举
 */
export const MonsterType = {
    /**
     * 普通怪物
     */
    NORMAL: {
        id: 1,
        name: "普通",
        description: "普通怪物"
    },
    /**
     * 不死怪物
     */
    UNDEAD: {
        id: 2,
        name: "undead",
        description: "undead 怪物"
    },
    /**
     * 野兽
     */
    BEAST: {
        id: 3,
        name: "野兽",
        description: "野兽"
    },
    /**
     * 人形生物
     */
    HUMANOID: {
        id: 4,
        name: "人形",
        description: "人形生物"
    },
    /**
     * 巨型生物
     */
    GIANT: {
        id: 5,
        name: "巨型",
        description: "巨型生物"
    },
    /**
     * 龙
     */
    DRAGON: {
        id: 6,
        name: "龙",
        description: "龙"
    },
    /**
     * 元素生物
     */
    ELEMENTAL: {
        id: 7,
        name: "元素",
        description: "元素生物"
    }
} as const as Record<string, MonsterType>;
