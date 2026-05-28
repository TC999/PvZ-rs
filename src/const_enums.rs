// 对应 C++ 中的 ConstEnums.h
// 游戏中的所有枚举类型定义

#![allow(non_camel_case_types, dead_code)]

// ============================================================
// Type aliases 对应 C++ 中的 typedef
// ============================================================
pub type AttachmentID = u32;
pub type CoinID = u32;
pub type ParticleID = u32;
pub type ParticleEmitterID = u32;
pub type ParticleSystemID = u32;
pub type PlantID = u32;
pub type ReanimationID = u32;
pub type ZombieID = u32;

pub const ATTACHMENTID_NULL: AttachmentID = 0;
pub const COINID_NULL: CoinID = 0;
pub const PARTICLEID_NULL: ParticleID = 0;
pub const PARTICLEEMITTERID_NULL: ParticleEmitterID = 0;
pub const PARTICLESYSTEMID_NULL: ParticleSystemID = 0;
pub const PLANTID_NULL: PlantID = 0;
pub const REANIMATIONID_NULL: ReanimationID = 0;
pub const ZOMBIEID_NULL: ZombieID = 0;

// ============================================================
// Enums
// ============================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum AdviceType {
    None = -1,
    ClickOnSun = 0,
    ClickedOnSun = 1,
    ClickedOnCoin = 2,
    SeedRefresh = 3,
    CantAffordPlant = 4,
    PlantGravebustersOnGraves = 5,
    PlantLilypadOnWater = 6,
    PlantTanglekelpOnWater = 7,
    PlantSeashroomOnWater = 8,
    PlantPotatoeMineOnLily = 9,
    PlantWrongArtType = 10,
    PlantNeedPot = 11,
    PlantNotOnGrave = 12,
    PlantNotOnCrater = 13,
    CantPlantThere = 14,
    PlantNotOnWater = 15,
    PlantingNeedsGround = 16,
    BeghouledDragToMatch3 = 17,
    BeghouledMatch3 = 18,
    BeghouledMatch4 = 19,
    BeghouledSaveSun = 20,
    BeghouledUseCrater1 = 21,
    BeghouledUseCrater2 = 22,
    PlantNotPassedLine = 23,
    PlantOnlyOnRepeaters = 24,
    PlantOnlyOnMelonpult = 25,
    PlantOnlyOnSunflower = 26,
    PlantOnlyOnSpikeweed = 27,
    PlantOnlyOnKernelpult = 28,
    PlantOnlyOnMagnetshroom = 29,
    PlantOnlyOnFumeshroom = 30,
    PlantOnlyOnLilypad = 31,
    PlantNeedsRepeater = 32,
    PlantNeedsMelonpult = 33,
    PlantNeedsSunflower = 34,
    PlantNeedsSpikeweed = 35,
    PlantNeedsKernelpult = 36,
    PlantNeedsMagnetshroom = 37,
    PlantNeedsFumeshroom = 38,
    PlantNeedsLilypad = 39,
    SlotMachinePull = 40,
    HugeWave = 41,
    ShovelRefresh = 42,
    PortalRelocating = 43,
    SlotMachineCollectSun = 44,
    DestroyPotsToFinishLevel = 45,
    UseShovelOnPots = 46,
    AlmostThere = 47,
    ZombiquariumClickTrophy = 48,
    ZombiquariumCollectSun = 49,
    ZombiquariumClickToFeed = 50,
    ZombiquariumBuySnorkel = 51,
    IZombiePlantsNotReal = 52,
    IZombieNotPassedLine = 53,
    IZombieLeftOfLine = 54,
    SlotMachineSpinAgain = 55,
    IZombieEatAllBrains = 56,
    PeashooterDied = 57,
    StinkySleeping = 58,
    BeghouledNoMoves = 59,
    PlantSunflower5 = 60,
    PlantingNeedSleeping = 61,
    ClickToContinue = 62,
    SurviveFlags = 63,
    UnlockedMode = 64,
    NeedWheelbarrow = 65,
    NUM_ADVICE_TYPES = 66,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AlmanacPage {
    Index = 0,
    Plants = 1,
    Zombies = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum AwardType {
    ForLevel = 0,
    CreditsZombieNote = 1,
    HelpZombieNote = 2,
    AchievementOnly = 3,
    PreCreditsZombieNote = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BackgroundType {
    Day = 0,
    Night = 1,
    Pool = 2,
    Fog = 3,
    Roof = 4,
    Boss = 5,
    MushroomGarden = 6,
    Greenhouse = 7,
    Zombiquarium = 8,
    TreeOfWisdom = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BoardResult {
    None = 0,
    Won = 1,
    Lost = 2,
    Restart = 3,
    Quit = 4,
    QuitApp = 5,
    Cheat = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum BossPart {
    BackLeg = 0,
    FrontLeg = 1,
    Main = 2,
    BackArm = 3,
    Fireball = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChallengePage {
    Survival = 0,
    Challenge = 1,
    Limbo = 2,
    Puzzle = 3,
    MAX_CHALLENGE_PAGES = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChallengeState {
    Normal = 0,
    BeghouledMoving = 1,
    BeghouledFalling = 2,
    BeghouledNoMatches = 3,
    SlotMachineRolling = 4,
    StormFlash1 = 5,
    StormFlash2 = 6,
    StormFlash3 = 7,
    ZenFading = 8,
    ScaryPotterMalleting = 9,
    LastStandOnslaught = 10,
    TreeJustGrew = 11,
    TreeGiveWisdom = 12,
    TreeWaitingToBabble = 13,
    TreeBabbling = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ChosenSeedState {
    FlyingToBank = 0,
    InBank = 1,
    FlyingToChooser = 2,
    InChooser = 3,
    PacketHidden = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CoinMotion {
    FromSky = 0,
    FromSkySlow = 1,
    FromPlant = 2,
    Coin = 3,
    LawnmowerCoin = 4,
    FromPresent = 5,
    FromBoss = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum CoinType {
    None = 0,
    Silver = 1,
    Gold = 2,
    Diamond = 3,
    Sun = 4,
    SmallSun = 5,
    LargeSun = 6,
    FinalSeedPacket = 7,
    Trophy = 8,
    Shovel = 9,
    Almanac = 10,
    CarKeys = 11,
    Vase = 12,
    WateringCan = 13,
    Taco = 14,
    Note = 15,
    UsableSeedPacket = 16,
    PresentPlant = 17,
    AwardMoneyBag = 18,
    AwardPresent = 19,
    AwardBagDiamond = 20,
    AwardSilverSunflower = 21,
    AwardGoldSunflower = 22,
    Chocolate = 23,
    AwardChocolate = 24,
    PresentMinigames = 25,
    PresentPuzzleMode = 26,
    PresentSurvivalMode = 27,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CrazyDaveState {
    Off = 0,
    Entering = 1,
    Leaving = 2,
    Idling = 3,
    Talking = 4,
    HandingTalking = 5,
    HandingIdling = 6,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum CursorType {
    Normal = 0,
    PlantFromBank = 1,
    PlantFromUsableCoin = 2,
    PlantFromGlove = 3,
    PlantFromDuplicator = 4,
    PlantFromWheelBarrow = 5,
    Shovel = 6,
    Hammer = 7,
    CobcannonTarget = 8,
    WateringCan = 9,
    Fertilizer = 10,
    BugSpray = 11,
    Phonograph = 12,
    Chocolate = 13,
    Glove = 14,
    MoneySign = 15,
    Wheelbarrow = 16,
    TreeFood = 17,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DamageFlags {
    BypassesShield = 0,
    HitsShieldAndBody = 1,
    Freeze = 2,
    DoesntCauseFlash = 3,
    DoesntLeaveBody = 4,
    Spike = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DamageRangeFlags {
    Ground = 0,
    Flying = 1,
    Submerged = 2,
    Dog = 3,
    OffGround = 4,
    Dying = 5,
    Underground = 6,
    OnlyMindControlled = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum Dialogs {
    NewGame = 0,
    Options = 1,
    NewOptions = 2,
    Almanac = 3,
    Store = 4,
    PreGameNag = 5,
    LoadGame = 6,
    ConfirmUpdateCheck = 7,
    CheckingUpdates = 8,
    RegisterError = 9,
    ColorDepthExp = 10,
    OpenUrlWait = 11,
    OpenUrlFail = 12,
    Quit = 13,
    HighScores = 14,
    Nag = 15,
    Info = 16,
    GameOver = 17,
    LevelComplete = 18,
    Paused = 19,
    NoMoreMoney = 20,
    Bonus = 21,
    ConfirmBackToMain = 22,
    ConfirmRestart = 23,
    ThanksForRegistering = 24,
    NotEnoughMoney = 25,
    Upgraded = 26,
    NoUpgrade = 27,
    ChooserWarning = 28,
    UserDialog = 29,
    CreateUser = 30,
    ConfirmDeleteUser = 31,
    RenameUser = 32,
    CreateUserError = 33,
    RenameUserError = 34,
    Cheat = 35,
    CheatError = 36,
    Continue = 37,
    GetReady = 38,
    RestartConfirm = 39,
    ConfirmPurchase = 40,
    ConfirmSell = 41,
    TimeUp = 42,
    VirtualHelp = 43,
    JumpAhead = 44,
    CrazyDave = 45,
    StorePurchase = 46,
    ZenSell = 47,
    Message = 48,
    Imitater = 49,
    PurchasePacketSlot = 50,
    NUM_DIALOGS = 51,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DebugTextMode {
    None = 0,
    ZombieSpawn = 1,
    Music = 2,
    Memory = 3,
    Collision = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DrawStringJustification {
    AlignLeft = 0,
    AlignRight = 1,
    AlignCenter = 2,
    AlignLeftVerticalMiddle = 3,
    AlignRightVerticalMiddle = 4,
    AlignCenterVerticalMiddle = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum DrawVariation {
    Normal = 0,
    Imitater = 1,
    MarigoldWhite = 2,
    MarigoldMagenta = 3,
    MarigoldOrange = 4,
    MarigoldPink = 5,
    MarigoldLightBlue = 6,
    MarigoldRed = 7,
    MarigoldBlue = 8,
    MarigoldViolet = 9,
    MarigoldLavender = 10,
    MarigoldYellow = 11,
    MarigoldLightGreen = 12,
    ZenGarden = 13,
    ZenGardenWater = 14,
    SproutNoFlower = 15,
    ImitaterLess = 16,
    Aquarium = 17,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EffectType {
    Particle = 0,
    Trail = 1,
    Reanim = 2,
    Attachment = 3,
    Other = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum EmitterType {
    Circle = 0,
    Box = 1,
    BoxPath = 2,
    CirclePath = 3,
    CircleEvenSpacing = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GameMode {
    Adventure = 0,
    SurvivalNormalStage1 = 1,
    SurvivalNormalStage2 = 2,
    SurvivalNormalStage3 = 3,
    SurvivalNormalStage4 = 4,
    SurvivalNormalStage5 = 5,
    SurvivalHardStage1 = 6,
    SurvivalHardStage2 = 7,
    SurvivalHardStage3 = 8,
    SurvivalHardStage4 = 9,
    SurvivalHardStage5 = 10,
    SurvivalEndlessStage1 = 11,
    SurvivalEndlessStage2 = 12,
    SurvivalEndlessStage3 = 13,
    SurvivalEndlessStage4 = 14,
    SurvivalEndlessStage5 = 15,
    ChallengeWarAndPeas = 16,
    ChallengeWallnutBowling = 17,
    ChallengeSlotMachine = 18,
    ChallengeRainingSeeds = 19,
    ChallengeBeghouled = 20,
    ChallengeInvisighoul = 21,
    ChallengeSeeingStars = 22,
    ChallengeZombiquarium = 23,
    ChallengeBeghouledTwist = 24,
    ChallengeLittleTrouble = 25,
    ChallengePortalCombat = 26,
    ChallengeColumn = 27,
    ChallengeBobsledBonanza = 28,
    ChallengeSpeed = 29,
    ChallengeWhackAZombie = 30,
    ChallengeLastStand = 31,
    ChallengeWarAndPeas2 = 32,
    ChallengeWallnutBowling2 = 33,
    ChallengePogoParty = 34,
    ChallengeFinalBoss = 35,
    ChallengeArtChallengeWallnut = 36,
    ChallengeSunnyDay = 37,
    ChallengeResodded = 38,
    ChallengeBigTime = 39,
    ChallengeArtChallengeSunflower = 40,
    ChallengeAirRaid = 41,
    ChallengeIce = 42,
    ChallengeZenGarden = 43,
    ChallengeHighGravity = 44,
    ChallengeGraveDanger = 45,
    ChallengeShovel = 46,
    ChallengeStormyNight = 47,
    ChallengeBungeeBlitz = 48,
    ChallengeSquirrel = 49,
    TreeOfWisdom = 50,
    ScaryPotter1 = 51,
    ScaryPotter2 = 52,
    ScaryPotter3 = 53,
    ScaryPotter4 = 54,
    ScaryPotter5 = 55,
    ScaryPotter6 = 56,
    ScaryPotter7 = 57,
    ScaryPotter8 = 58,
    ScaryPotter9 = 59,
    ScaryPotterEndless = 60,
    PuzzleIZombie1 = 61,
    PuzzleIZombie2 = 62,
    PuzzleIZombie3 = 63,
    PuzzleIZombie4 = 64,
    PuzzleIZombie5 = 65,
    PuzzleIZombie6 = 66,
    PuzzleIZombie7 = 67,
    PuzzleIZombie8 = 68,
    PuzzleIZombie9 = 69,
    PuzzleIZombieEndless = 70,
    Upsell = 71,
    Intro = 72,
    NUM_GAME_MODES = 73,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum GameObjectType {
    None = 0,
    Plant = 1,
    Projectile = 2,
    Coin = 3,
    SeedPacket = 4,
    Shovel = 5,
    WateringCan = 6,
    Fertilizer = 7,
    BugSpray = 8,
    Phonograph = 9,
    Chocolate = 10,
    Glove = 11,
    MoneySign = 12,
    Wheelbarrow = 13,
    TreeFood = 14,
    NextGarden = 15,
    MenuButton = 16,
    StoreButton = 17,
    SlotMachineHandle = 18,
    ScaryPot = 19,
    Stinky = 20,
    TreeOfWisdom = 21,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GameScenes {
    Loading = 0,
    Menu = 1,
    LevelIntro = 2,
    Playing = 3,
    ZombiesWon = 4,
    Award = 5,
    Credit = 6,
    Challenge = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GardenType {
    Main = 0,
    Mushroom = 1,
    Wheelbarrow = 2,
    Aquarium = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GridItemType {
    None = 0,
    Gravestone = 1,
    Crater = 2,
    Ladder = 3,
    PortalCircle = 4,
    PortalSquare = 5,
    Brain = 6,
    ScaryPot = 7,
    ZenTool = 9,
    Stinky = 10,
    Rake = 11,
    IZombieBrain = 12,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GridItemState {
    Normal = 0,
    GravestoneSpecial = 1,
    PortalClosed = 2,
    ScaryPotQuestion = 3,
    ScaryPotLeaf = 4,
    ScaryPotZombie = 5,
    SquirrelWaiting = 6,
    SquirrelPeeking = 7,
    SquirrelRunningUp = 8,
    SquirrelRunningDown = 9,
    SquirrelRunningLeft = 10,
    SquirrelRunningRight = 11,
    SquirrelCaught = 12,
    SquirrelZombie = 13,
    ZenToolWateringCan = 14,
    ZenToolFertilizer = 15,
    ZenToolBugSpray = 16,
    ZenToolPhonograph = 17,
    ZenToolGoldWateringCan = 18,
    StinkyWalkingLeft = 19,
    StinkyTurningLeft = 20,
    StinkyWalkingRight = 21,
    StinkyTurningRight = 22,
    StinkySleeping = 23,
    StinkyFallingAsleep = 24,
    StinkyWakingUp = 25,
    RakeAttracting = 26,
    RakeWaiting = 27,
    RakeTriggered = 28,
    BrainSquished = 29,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum GridSquareType {
    None = 0,
    Grass = 1,
    Dirt = 2,
    Pool = 3,
    HighGround = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum HelmType {
    None = 0,
    TrafficCone = 1,
    Pail = 2,
    Football = 3,
    Digger = 4,
    RedEyes = 5,
    Headband = 6,
    Bobsled = 7,
    Wallnut = 8,
    Tallnut = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LawnMowerState {
    RollingIn = 0,
    Ready = 1,
    Triggered = 2,
    Squished = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum LawnMowerType {
    Lawn = 0,
    Pool = 1,
    Roof = 2,
    SuperMower = 3,
    NUM_MOWER_TYPES = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MessageStyle {
    Off = 0,
    TutorialLevel1 = 1,
    TutorialLevel1Stay = 2,
    TutorialLevel2 = 3,
    TutorialLater = 4,
    TutorialLaterStay = 5,
    HintLong = 6,
    HintFast = 7,
    HintStay = 8,
    HintTallFast = 9,
    HintTallUnlockMessage = 10,
    HintTallLong = 11,
    BigMiddle = 12,
    BigMiddleFast = 13,
    HouseName = 14,
    HugeWave = 15,
    SlotMachine = 16,
    ZenGardenLong = 17,
    Achievement = 18,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum MowerHeight {
    Land = 0,
    DownToPool = 1,
    InPool = 2,
    UpToLand = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum NotRecommend {
    Nocturnal = 0,
    NeedsPool = 1,
    NeedsGraves = 2,
    NeedsFog = 3,
    NeedsRoof = 4,
    OnRoof = 5,
    ForChallenge = 6,
    AtNight = 7,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ParticleEffect {
    None = -1,
    MelonSplash = 0,
    WinterMelon = 1,
    FumeCloud = 2,
    PopcornSplash = 3,
    Powie = 4,
    JackExplode = 5,
    ZombieHead = 6,
    ZombieArm = 7,
    ZombieTrafficCone = 8,
    ZombiePail = 9,
    ZombieHelmet = 10,
    ZombieFlag = 11,
    ZombieDoor = 12,
    ZombieNewspaper = 13,
    ZombieHeadlight = 14,
    Pow = 15,
    ZombiePogo = 16,
    ZombieNewspaperHead = 17,
    ZombieBalloonHead = 18,
    SodRoll = 19,
    GraveStoneRise = 20,
    Planting = 21,
    PlantingPool = 22,
    ZombieRise = 23,
    GraveBuster = 24,
    GraveBusterDie = 25,
    PoolSplash = 26,
    IceSparkle = 27,
    SeedPacket = 28,
    TallNutBlock = 29,
    Doom = 30,
    DiggerRise = 31,
    DiggerTunnel = 32,
    DancerRise = 33,
    PoolSparkly = 34,
    WallnutEatSmall = 35,
    WallnutEatLarge = 36,
    PeaSplat = 37,
    ButterSplat = 38,
    CabbageSplat = 39,
    PuffSplat = 40,
    StarSplat = 41,
    IceTrap = 42,
    SnowpeaSplat = 43,
    SnowpeaPuff = 44,
    SnowpeaTrail = 45,
    LanternShine = 46,
    SeedPacketPickup = 47,
    PotatoMine = 48,
    PotatoMineRise = 49,
    PuffshroomTrail = 50,
    PuffshroomMuzzle = 51,
    SeedPacketFlash = 52,
    WhackAZombieRise = 53,
    ZombieLadder = 54,
    UmbrellaReflect = 55,
    SeedPacketPick = 56,
    IceTrapZombie = 57,
    IceTrapRelease = 58,
    ZamboniSmoke = 59,
    GloomCloud = 60,
    ZombiePogoHead = 61,
    ZamboniTire = 62,
    ZamboniExplosion = 63,
    ZamboniExplosion2 = 64,
    CatapultExplosion = 65,
    MowerCloud = 66,
    BossIceBall = 67,
    Blastmark = 68,
    CoinPickupArrow = 69,
    PresentPickup = 70,
    ImitaterMorph = 71,
    MoweredZombieHead = 72,
    MoweredZombieArm = 73,
    ZombieHeadPool = 74,
    ZombieBossFireball = 75,
    FireballDeath = 76,
    IceballDeath = 77,
    IceballTrail = 78,
    FireballTrail = 79,
    BossExplosion = 80,
    ScreenFlash = 81,
    TrophySparkle = 82,
    PortalCircle = 83,
    PortalSquare = 84,
    PottedPlantGlow = 85,
    PottedWaterPlantGlow = 86,
    PottedZenGlow = 87,
    MindControl = 88,
    VaseShatter = 89,
    VaseShatterLeaf = 90,
    VaseShatterZombie = 91,
    AwardPickupArrow = 92,
    ZombieSeaweed = 93,
    ZombieMustache = 94,
    ZombieSunglass = 95,
    ZombiePinata = 96,
    DustSquash = 97,
    DustFoot = 98,
    ZombieDaisies = 99,
    CreditStrobe = 100,
    CreditsRaySwipe = 101,
    CreditsZombieheadWipe = 102,
    Starburst = 103,
    CreditsFog = 104,
    PresetPickUpArrow = 105,
    NUM_PARTICLES = 106,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantPriority {
    EatingOrder = 0,
    DiggingOrder = 1,
    BungeeOrder = 2,
    CatapultOrder = 3,
    ZenToolOrder = 4,
    Any = 5,
    OnlyNormalPosition = 6,
    OnlyFlying = 7,
    OnlyPumpkin = 8,
    OnlyUnderPlant = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantingReason {
    Ok = 0,
    NotHere = 1,
    OnlyOnGraves = 2,
    OnlyInPool = 3,
    OnlyOnGround = 4,
    NeedsPot = 5,
    NotOnArt = 6,
    NotPassedLine = 7,
    NeedsUpgrade = 8,
    NotOnGrave = 9,
    NotOnCrater = 10,
    NotOnWater = 11,
    NeedsGround = 12,
    NeedsSleeping = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PlantRowType {
    Dirt = 0,
    Normal = 1,
    Pool = 2,
    HighGround = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PottedPlantAge {
    Sprout = 0,
    Small = 1,
    Medium = 2,
    Full = 3,
}

impl PottedPlantAge {
    pub fn next_age(self) -> Self {
        match self {
            PottedPlantAge::Sprout => PottedPlantAge::Small,
            PottedPlantAge::Small => PottedPlantAge::Medium,
            PottedPlantAge::Medium => PottedPlantAge::Full,
            PottedPlantAge::Full => PottedPlantAge::Full,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum PottedPlantNeed {
    None = 0,
    Water = 1,
    Fertilizer = 2,
    BugSpray = 3,
    Phonograph = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProjectileMotion {
    Straight = 0,
    Lobbed = 1,
    Threepeater = 2,
    Bee = 3,
    BeeBackwards = 4,
    Puff = 5,
    Backwards = 6,
    Star = 7,
    FloatOver = 8,
    Homing = 9,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ProjectileType {
    Pea = 0,
    Snowpea = 1,
    Cabbage = 2,
    Melon = 3,
    Puff = 4,
    WinterMelon = 5,
    Fireball = 6,
    Star = 7,
    Spike = 8,
    Basketball = 9,
    Kernel = 10,
    Cobbig = 11,
    Butter = 12,
    ZombiePea = 13,
    NUM_PROJECTILES = 14,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(u32)]
pub enum ReanimationType {
    None = u32::MAX,
    LoadbarSprout = 0,
    LoadbarZombiehead = 1,
    SodRoll = 2,
    FinalWave = 3,
    Peashooter = 4,
    Wallnut = 5,
    Lilypad = 6,
    Sunflower = 7,
    Lawnmower = 8,
    ReadySetPlant = 9,
    CherryBomb = 10,
    Squash = 11,
    Doomshroom = 12,
    Snowpea = 13,
    Repeater = 14,
    Sunshroom = 15,
    Tallnut = 16,
    Fumeshroom = 17,
    Puffshroom = 18,
    Hypnoshroom = 19,
    Chomper = 20,
    Zombie = 21,
    Sun = 22,
    PotatoMine = 23,
    Spikeweed = 24,
    Spikerock = 25,
    Threepeater = 26,
    Marigold = 27,
    Iceshroom = 28,
    ZombieFootball = 29,
    ZombieNewspaper = 30,
    ZombieZamboni = 31,
    Splash = 32,
    Jalapeno = 33,
    JalapenoFire = 34,
    CoinSilver = 35,
    ZombieCharred = 36,
    ZombieCharredImp = 37,
    ZombieCharredDigger = 38,
    ZombieCharredZamboni = 39,
    ZombieCharredCatapult = 40,
    ZombieCharredGargantuar = 41,
    Scaredyshroom = 42,
    Pumpkin = 43,
    Plantern = 44,
    Torchwood = 45,
    SplitPea = 46,
    Seashroom = 47,
    Blover = 48,
    FlowerPot = 49,
    Cactus = 50,
    Dancer = 51,
    Tanglekelp = 52,
    Starfruit = 53,
    Polevaulter = 54,
    Balloon = 55,
    Gargantuar = 56,
    Imp = 57,
    Digger = 58,
    DiggerDirt = 59,
    ZombieDolphinRider = 60,
    Pogo = 61,
    BackupDancer = 62,
    Bobsled = 63,
    JackInTheBox = 64,
    Snorkel = 65,
    Bungee = 66,
    Catapult = 67,
    Ladder = 68,
    Puff = 69,
    Sleeping = 70,
    GraveBuster = 71,
    ZombiesWon = 72,
    Magnetshroom = 73,
    Boss = 74,
    Cabbagepult = 75,
    Kernelpult = 76,
    Melonpult = 77,
    CoffeeBean = 78,
    UmbrellaLeaf = 79,
    GatlingPea = 80,
    Cattail = 81,
    Gloomshroom = 82,
    BossIceball = 83,
    BossFireball = 84,
    Cobcannon = 85,
    Garlic = 86,
    GoldMagnet = 87,
    WinterMelon = 88,
    TwinSunflower = 89,
    PoolCleaner = 90,
    RoofCleaner = 91,
    FirePea = 92,
    Imitater = 93,
    Yeti = 94,
    BossDriver = 95,
    LawnMoweredZombie = 96,
    CrazyDave = 97,
    TextFadeOn = 98,
    Hammer = 99,
    SlotMachineHandle = 100,
    CreditsFootball = 101,
    CreditsJackbox = 102,
    SelectorScreen = 103,
    PortalCircle = 104,
    PortalSquare = 105,
    ZenGardenSprout = 106,
    ZenGardenWateringCan = 107,
    ZenGardenFertilizer = 108,
    ZenGardenBugSpray = 109,
    ZenGardenPhonograph = 110,
    Diamond = 111,
    ZombieHand = 112,
    Stinky = 113,
    Rake = 114,
    RainCircle = 115,
    RainSplash = 116,
    ZombieSurprise = 117,
    CoinGold = 118,
    TreeOfWisdom = 119,
    TreeOfWisdomClouds = 120,
    TreeOfWisdomTreeFood = 121,
    CreditsMain = 122,
    CreditsMain2 = 123,
    CreditsMain3 = 124,
    ZombieCreditsDance = 125,
    CreditsStage = 126,
    CreditsBigBrain = 127,
    CreditsFlowerPetals = 128,
    CreditsInfantry = 129,
    CreditsThroat = 130,
    CreditsCrazyDave = 131,
    CreditsBossDance = 132,
    ZombieCreditsScreenDoor = 133,
    ZombieCreditsConehead = 134,
    CreditsZombieArmy1 = 135,
    CreditsZombieArmy2 = 136,
    CreditsTombstones = 137,
    CreditsSolarPower = 138,
    CreditsAnyHour = 139,
    CreditsWeAreTheUndead = 140,
    CreditsDiscoLights = 141,
    Flag = 142,
    NUM_REANIMS = 143,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ReanimLoopType {
    Loop = 0,
    LoopFullLastFrame = 1,
    PlayOnce = 2,
    PlayOnceAndHold = 3,
    PlayOnceFullLastFrame = 4,
    PlayOnceFullLastFrameAndHold = 5,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RenderLayer {
    RowOffset = 10000,
    UiBottom = 100000,
    Ground = 200000,
    Lawn = 300000,
    GraveStone = 301000,
    Plant = 302000,
    Zombie = 303000,
    Boss = 304000,
    Projectile = 305000,
    LawnMower = 306000,
    Particle = 307000,
    Top = 400000,
    Fog = 500000,
    CoinBank = 600000,
    UiTop = 700000,
    AboveUi = 800000,
    ScreenFade = 900000,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum RenderObjectType {
    Coin = 0,
    Projectile = 1,
    Zombie = 2,
    ZombieShadow = 3,
    ZombieBungeeTarget = 4,
    Plant = 5,
    PlantOverlay = 6,
    PlantMagnetItems = 7,
    CursorPreview = 8,
    Particle = 9,
    Reanimation = 10,
    Ice = 11,
    TopUi = 12,
    Fog = 13,
    Storm = 14,
    BottomUi = 15,
    Backdrop = 16,
    DoorMask = 17,
    CoinBank = 18,
    ProjectileShadow = 19,
    Mower = 20,
    ScreenFade = 21,
    BossPart = 22,
    GridItem = 23,
    GridItemOverlay = 24,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ScaryPotType {
    None = 0,
    Seed = 1,
    Zombie = 2,
    Sun = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum SeedChooserState {
    Normal = 0,
    ViewLawn = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum SeedType {
    Peashooter = 0,
    Sunflower = 1,
    CherryBomb = 2,
    Wallnut = 3,
    PotatoMine = 4,
    SnowPea = 5,
    Chomper = 6,
    Repeater = 7,
    Puffshroom = 8,
    Sunshroom = 9,
    Fumeshroom = 10,
    GraveBuster = 11,
    Hypnoshroom = 12,
    Scaredyshroom = 13,
    Iceshroom = 14,
    Doomshroom = 15,
    Lilypad = 16,
    Squash = 17,
    Threepeater = 18,
    Tanglekelp = 19,
    Jalapeno = 20,
    Spikeweed = 21,
    Torchwood = 22,
    Tallnut = 23,
    Seashroom = 24,
    Plantern = 25,
    Cactus = 26,
    Blover = 27,
    SplitPea = 28,
    Starfruit = 29,
    PumpkinShell = 30,
    Magnetshroom = 31,
    Cabbagepult = 32,
    FlowerPot = 33,
    Kernelpult = 34,
    InstantCoffee = 35,
    Garlic = 36,
    Umbrella = 37,
    Marigold = 38,
    Melonpult = 39,
    GatlingPea = 40,
    TwinSunflower = 41,
    Gloomshroom = 42,
    Cattail = 43,
    WinterMelon = 44,
    GoldMagnet = 45,
    Spikerock = 46,
    Cobcannon = 47,
    Imitater = 48,
    ExplodeONut = 49,
    GiantWallnut = 50,
    Sprout = 51,
    LeftPeater = 52,
    NUM_SEED_TYPES = 53,
    BeghouledButtonShuffle = 54,
    BeghouledButtonCrater = 55,
    SlotMachineSun = 56,
    SlotMachineDiamond = 57,
    ZombiquariumSnorkle = 58,
    ZombiquariumTrophy = 59,
    ZombieNormal = 60,
    ZombieTrafficCone = 61,
    ZombiePolevaulter = 62,
    ZombiePail = 63,
    ZombieLadder = 64,
    ZombieDigger = 65,
    ZombieBungee = 66,
    ZombieFootball = 67,
    ZombieBalloon = 68,
    ZombieScreenDoor = 69,
    Zomboni = 70,
    ZombiePogo = 71,
    ZombieDancer = 72,
    ZombieGargantuar = 73,
    ZombieImp = 74,
    None = -1,
}

// NUM_SEEDS_IN_CHOOSER 作为独立常量，避免枚举值冲突
pub const NUM_SEEDS_IN_CHOOSER: i32 = 49;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ShieldType {
    None = 0,
    Door = 1,
    Newspaper = 2,
    Ladder = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StoreItem {
    PlantGatlingPea = 0,
    PlantTwinSunflower = 1,
    PlantGloomshroom = 2,
    PlantCattail = 3,
    PlantWinterMelon = 4,
    PlantGoldMagnet = 5,
    PlantSpikerock = 6,
    PlantCobcannon = 7,
    PlantImitater = 8,
    BonusLawnMower = 9,
    PottedMarigold1 = 10,
    PottedMarigold2 = 11,
    PottedMarigold3 = 12,
    GoldWateringCan = 13,
    Fertilizer = 14,
    BugSpray = 15,
    Phonograph = 16,
    GardeningGlove = 17,
    MushroomGarden = 18,
    WheelBarrow = 19,
    StinkyTheSnail = 20,
    PacketUpgrade = 21,
    PoolCleaner = 22,
    RoofCleaner = 23,
    Rake = 24,
    AquariumGarden = 25,
    Chocolate = 26,
    TreeOfWisdom = 27,
    TreeFood = 28,
    FirstAid = 29,
    Pvz = 30,
    Invalid = -1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum StorePages {
    SlotUpgrades = 0,
    PlantUpgrades = 1,
    Zen1 = 2,
    Zen2 = 3,
    NUM_STORE_PAGES = 4,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TodCurves {
    Constant = 0,
    Linear = 1,
    EaseIn = 2,
    EaseOut = 3,
    EaseInOut = 4,
    EaseInOutWeak = 5,
    FastInOut = 6,
    FastInOutWeak = 7,
    WeakFastInOut = 8,
    Bounce = 9,
    BounceFastMiddle = 10,
    BounceSlowMiddle = 11,
    SinWave = 12,
    EaseSinWave = 13,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TrialType {
    None = 0,
    StageLocked = 1,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum TutorialState {
    Off = 0,
    Level1PickUpPeashooter = 1,
    Level1PlantPeashooter = 2,
    Level1RefreshPeashooter = 3,
    Level1Completed = 4,
    Level2PickUpSunflower = 5,
    Level2PlantSunflower = 6,
    Level2RefreshSunflower = 7,
    Level2Completed = 8,
    MoreSunPickUpSunflower = 9,
    MoreSunPlantSunflower = 10,
    MoreSunRefreshSunflower = 11,
    MoreSunCompleted = 12,
    SlotMachinePull = 13,
    SlotMachineCompleted = 14,
    ShovelPickup = 15,
    ShovelDig = 16,
    ShovelKeepDigging = 17,
    ShovelCompleted = 18,
    ZombiquariumBuySnorkel = 19,
    ZombiquariumBoughtSnorkel = 20,
    ZombiquariumClickTrophy = 21,
    ZenGardenPickupWater = 22,
    ZenGardenWaterPlant = 23,
    ZenGardenKeepWatering = 24,
    ZenGardenVisitStore = 25,
    ZenGardenFertilizePlants = 26,
    ZenGardenCompleted = 27,
    WhackAZombieBeforePickSeed = 28,
    WhackAZombiePickSeed = 29,
    WhackAZombieCompleted = 30,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum UnlockingState {
    Off = 0,
    Shaking = 1,
    Fading = 2,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombieHeight {
    Normal = 0,
    InToPool = 1,
    OutOfPool = 2,
    DraggedUnder = 3,
    UpToHighGround = 4,
    DownOffHighGround = 5,
    UpLadder = 6,
    Falling = 7,
    InToChimney = 8,
    GettingBungeeDropped = 9,
    Zombiquarium = 10,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i32)]
pub enum ZombiePhase {
    Normal = 0,
    Dying = 1,
    Burned = 2,
    Mowered = 3,
    BungeeDiving = 4,
    BungeeDivingScreaming = 5,
    BungeeAtBottom = 6,
    BungeeGrabbing = 7,
    BungeeRising = 8,
    BungeeHitOuchy = 9,
    BungeeCutscene = 10,
    PolevaulterPreVault = 11,
    PolevaulterInVault = 12,
    PolevaulterPostVault = 13,
    RisingFromGrave = 14,
    JackInTheBoxRunning = 15,
    JackInTheBoxPopping = 16,
    BobsledSliding = 17,
    BobsledBoarding = 18,
    BobsledCrashing = 19,
    PogoBouncing = 20,
    PogoHighBounce1 = 21,
    PogoHighBounce2 = 22,
    PogoHighBounce3 = 23,
    PogoHighBounce4 = 24,
    PogoHighBounce5 = 25,
    PogoHighBounce6 = 26,
    PogoForwardBounce2 = 27,
    PogoForwardBounce7 = 28,
    NewspaperReading = 29,
    NewspaperMaddening = 30,
    NewspaperMad = 31,
    DiggerTunneling = 32,
    DiggerRising = 33,
    DiggerTunnelingPauseWithoutAxe = 34,
    DiggerRiseWithoutAxe = 35,
    DiggerStunned = 36,
    DiggerWalking = 37,
    DiggerWalkingWithoutAxe = 38,
    DiggerCutscene = 39,
    DancerDancingIn = 40,
    DancerSnappingFingers = 41,
    DancerSnappingFingersWithLight = 42,
    DancerSnappingFingersHold = 43,
    DancerDancingLeft = 44,
    DancerWalkToRaise = 45,
    DancerRaiseLeft1 = 46,
    DancerRaiseRight1 = 47,
    DancerRaiseLeft2 = 48,
    DancerRaiseRight2 = 49,
    DancerRising = 50,
    DolphinWalking = 51,
    DolphinIntoPool = 52,
    DolphinRiding = 53,
    DolphinInJump = 54,
    DolphinWalkingInPool = 55,
    DolphinWalkingWithoutDolphin = 56,
    SnorkelWalking = 57,
    SnorkelIntoPool = 58,
    SnorkelWalkingInPool = 59,
    SnorkelUpToEat = 60,
    SnorkelEatingInPool = 61,
    SnorkelDownFromEat = 62,
    ZombiquariumAccel = 63,
    ZombiquariumDrift = 64,
    ZombiquariumBackAndForth = 65,
    ZombiquariumBite = 66,
    CatapultLaunching = 67,
    CatapultReloading = 68,
    GargantuarThrowing = 69,
    GargantuarSmashing = 70,
    ImpGettingThrown = 71,
    ImpLanding = 72,
    BalloonFlying = 73,
    BalloonPopping = 74,
    BalloonWalking = 75,
    LadderCarrying = 76,
    LadderPlacing = 77,
    BossEnter = 78,
    BossIdle = 79,
    BossSpawning = 80,
    BossStomping = 81,
    BossBungeesEnter = 82,
    BossBungeesDrop = 83,
    BossBungeesLeave = 84,
    BossDropRv = 85,
    BossHeadEnter = 86,
    BossHeadIdleBeforeSpit = 87,
    BossHeadIdleAfterSpit = 88,
    BossHeadSpit = 89,
    BossHeadLeave = 90,
    YetiRunning = 91,
    SquashPreLaunch = 92,
    SquashRising = 93,
    SquashFalling = 94,
    SquashDoneFalling = 95,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[repr(i32)]
pub enum ZombieType {
    Invalid = -1,
    Normal = 0,
    Flag = 1,
    TrafficCone = 2,
    Polevaulter = 3,
    Pail = 4,
    Newspaper = 5,
    Door = 6,
    Football = 7,
    Dancer = 8,
    BackupDancer = 9,
    DuckyTube = 10,
    Snorkel = 11,
    Zamboni = 12,
    Bobsled = 13,
    DolphinRider = 14,
    JackInTheBox = 15,
    Balloon = 16,
    Digger = 17,
    Pogo = 18,
    Yeti = 19,
    Bungee = 20,
    Ladder = 21,
    Catapult = 22,
    Gargantuar = 23,
    Imp = 24,
    Boss = 25,
    PeaHead = 26,
    WallnutHead = 27,
    JalapenoHead = 28,
    GatlingHead = 29,
    SquashHead = 30,
    TallnutHead = 31,
    RedEyeGargantuar = 32,
    NUM_ZOMBIE_TYPES = 33,
    CachedPolevaulterWithPole = 34,
    NUM_CACHED_ZOMBIE_TYPES = 35,
}
