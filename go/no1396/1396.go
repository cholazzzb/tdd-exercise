package no1396

type Customer struct {
	stationName string
	startTime   int
}

type UndergroundSystem struct {
	position map[int]Customer // map[id]stationName
	travels  map[string][]int // map[stationName(start)-stationName(end)][]t
}

func Constructor() UndergroundSystem {
	return UndergroundSystem{
		position: map[int]Customer{},
		travels:  make(map[string][]int),
	}
}

func (us *UndergroundSystem) CreateKey(startStationName, endStationName string) string {
	return startStationName + "-" + endStationName
}

func (us *UndergroundSystem) CheckIn(id int, stationName string, t int) {
	if _, ok := us.position[id]; ok {
		panic("the member already inside the station")
	}
	us.position[id] = Customer{stationName, t}
}

func (us *UndergroundSystem) CheckOut(id int, stationName string, t int) {
	if _, ok := us.position[id]; !ok {
		panic("the member are not inside the station")
	}
	startStationName := us.position[id].stationName
	key := us.CreateKey(startStationName, stationName)
	startTime := us.position[id].startTime
	_, ok := us.travels[key]
	if ok {
		us.travels[key] = append(us.travels[key], t-startTime)
	} else {
		us.travels[key] = []int{t - startTime}
	}
	delete(us.position, id)
}

func (us *UndergroundSystem) GetAverageTime(startStation string, endStation string) float64 {
	key := us.CreateKey(startStation, endStation)
	if travels, ok := us.travels[key]; ok {
		avg := 0.0
		for _, trav := range travels {
			avg += float64(trav)
		}
		return avg / float64(len(travels))
	}
	panic("travels key are not found")
}
