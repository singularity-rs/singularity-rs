## Template

- [ ] Entities
    - [ ] Component
<br/><br/>
    - [ ] Optional Component
<br/><br/>
    - [ ] Component (not MVP)


## General, Core Game functionality

- [ ] Unit
    - [ ] Moving
        - [ ] Position
        - [ ] Velocity
        - [ ] Acceleration
    - [ ] UnitType (Produce, Carry, Build, Defend)
    - [ ] Graph-Based Pathfinding module
    - [ ] Communication: Getting Tasks (finishing, failing, impossible to do, ...)
    - [ ] Drawable
<br/><br/>
    - [ ] ResourceCarrying
    - [ ] ActiveTask

<br/><br/>

- [ ] Tasks
    - [ ] JobType
    - [ ] BeginPlatform
    - [ ] EndPlatform
    - [ ] DoWorkForPlatformAction
    - [ ] ResourceType


<br/><br/>

- [ ] FreeMovingUnit
    - [ ] Moving
        - [ ] Position
        - [ ] Velocity
        - [ ] Acceleration
    - [ ] A*-based pathfinding
    - [ ] Flocking / FlockingGroups
        - [ ] FlockingGroupNumber
        - [ ] GroupMiddle
        - [ ] GroupAverage
    - [ ] TeamNumber (own or enemy/other faction)
    - [ ] ReportState
    - [ ] Selectable
    - [ ] Drawable
    - [ ] LoseHealth
<br/><br/>
    - [ ] Shooting
    - [ ] TargetingAt
    - [ ] UserControl
    - [ ] AIControl
<br/><br/>
    - [ ] Settler (not MVP)
    - [ ] Transporter (not MVP)
    - [ ] Communicator (not MVP)


<br/><br/>

- [ ] StoryManager
    - [ ] Level
        - [ ] Goals
    - [ ] Map
        - [ ] Fog Of War
        - [ ] CollisionMap
            - [ ] Collidable objects on Map
        - [ ] ResourceMap
        - [ ] StructureMap
        - [ ] UnitMap
        - [ ] Camera (? -> put that somewhere else !!)


<br/><br/>

- [ ] Graph
    - [ ] Nodes
    - [ ] Edges


<br/><br/>

- [ ] Platform
    - [ ] PlatformType
    - [ ] Position
    - [ ] ResourcesOnPlatform
    - [ ] IsBluePrint
    - [ ] Selectable
    - [ ] ReportState
    - [ ] PauseActions
    - [ ] LoseHealth
    - [ ] Drawable
    - [ ] ConnectedPlatforms
    - [ ] RoutingTable
<br/><br/>
    - [ ] ProduceResource<Which>
    - [ ] RequestResources
    - [ ] RemoveResource
    - [ ] CreateResource
    - [ ] DefendWithBullets
    - [ ] DefendWithLaser
    - [ ] RequestUnits
<br/><br/>
    - [ ] DefendWithRockets (not MVP)
    - [ ] Communicator (not MVP)
    - [ ] CreateEnergy (not MVP)
    - [ ] StoreResource (not MVP) (- [ ]> Logistics Hub)
    - [ ] CreateUnit (not MVP)
    - [ ] CreateFreeMovingUnit (not MVP)


<br/><br/>

## User Interface

- [ ] GraphUnitStats
    - [ ] Position
    - [ ] Size
    - [ ] Label(s)
    - [ ] GraphSelector
    - [ ] UnitTypeField
        - [ ] Label
        - [ ] HorizontalSlider
        - [ ] Number


<br/><br/>

- [ ] ResourceStats
    - [ ] Position
    - [ ] Size
    - [ ] ResourceDataCounter
    - [ ] ResourcePresenter
        - [ ] ResourceGraphic
        - [ ] NameLabel
        - [ ] NumberLabel
        - [ ] DeltaLabel
    - [ ] VerticalSlider


<br/><br/>

- [ ] SelectionStats
    - [ ] Position
    - [ ] Size
    - [ ] Label
    - [ ] ResourcePresenter
        - [ ] ResourceGraphic
        - [ ] NameLabel
        - [ ] NumberLabel
    - [ ] ActionPresenter
        - [ ] ActionGraphic
        - [ ] ActionLabel
        - [ ] ResourceReqGraphics
    - [ ] UnitPresenter
        - [ ] UnitTaskGraphic
        - [ ] ...
    - [ ] FreeMovingUnitPresenter
        - [ ] UnitGraphic
        - [ ] ...


<br/><br/>

- [ ] EventLog
    - [ ] Position
    - [ ] Size
    - [ ] ...


<br/><br/>

- [ ] NextGoals
    - [ ] Position
    - [ ] Size
    - [ ] ...


<br/><br/>

- [ ] BuildMenu
    - [ ] Position
    - [ ] Size
    - [ ] ...


<br/><br/>

- [ ] MiniMap (not MVP)
    - [ ] Position
    - [ ] Size
    - [ ] ...









<br/><br/>
<br/><br/>


## Systems

- [ ] Systems
    - [ ] Setting Blueprints
    - [ ] Building Blueprints
    - [ ] Selecting Platforms
    - [ ] Modifying values on Platforms
    - [ ] Changing Sliders for how many units should do what on that Graph
    - [ ] Updating all the UI elements
    - [ ] Moving & adjusting Camera
    - [ ] Pause / Unpause Game
    - [ ] Update on achieved goals/trigger next goals/trigger Enemies coming/ ....
<br/><br/>
    - [ ] Moving (Free-) Units and Resources
    - [ ] Do Path-finding for these needed
    - [ ] Flocking: Compute direction averages
    - [ ] Flocking: Compute middle points
    - [ ] Flocking: Compute distance between units
<br/><br/>
    - [ ] Produce & Consume Resources
    - [ ] Transport Resources
    - [ ] Provide new Tasks for Units asking for them -> DistributionManager
    - [ ] Calculate flowgraphs of resources (async?)
<br/><br/>
    - [ ] 'StoryManager' for advancing the story / levels upon reaching goals
    - [ ] Planning Military maneuvers from the AI
    - [ ] Planning Building and Resource aquirenment from the AI
<br/><br/>
    - [ ] JoinGraphs
    - [ ] SplitGraphs
    - [ ] AddToGraph




<br/><br/>
<br/><br/>

## Other stuff

- [ ] Flocking (https://en.wikipedia.org/wiki/Boids, https://github.com/henninglive/boids-rs/blob/master/src/main.rs )
- [ ] Agent-like behaviour
- [ ] Behaviour-trees
- [ ] Model-View-Controller (splitting)
- [ ] Procedural Map generation: http://pcg.wikidot.com/category-pcg-algorithms
- [ ] Blueprint Editor
- [ ] Everything not in MVP
- [ ] Blueprint Editor



