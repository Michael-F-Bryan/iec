FUNCTION mqttGetNextMessageID : INT
BEGIN
    mqttData.nextMsgId := mqttData.nextMsgId + 1;
    
    IF (mqttData.nextMsgId <= 0) THEN
        mqttData.nextMsgId := 1;
    END_IF;
    
    mqttGetNextMessageID := mqttData.nextMsgId;
END_FUNCTION
