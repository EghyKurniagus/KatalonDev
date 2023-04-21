<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>RetrieveApprovals</name>
   <tag></tag>
   <elementGuidId>07c8ef69-4ea5-435a-986b-cb03eca52684</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <authorizationRequest>
      <authorizationInfo>
         <entry>
            <key>bearerToken</key>
            <value>eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1bmlxdWVfbmFtZSI6IkFETUlOIiwicm9sZSI6IkFkbWluaXN0YXJ0b3IiLCJodHRwOi8vc2NoZW1hcy5taWNyb3NvZnQuY29tL3dzLzIwMDgvMDYvaWRlbnRpdHkvY2xhaW1zL3VzZXJkYXRhIjoie1widXNlcl9uYW1lXCI6XCJBRE1JTlwiLFwicGFzc3dvcmRcIjpcIkFkbWluXCIsXCJwb3NpdGlvbklkXCI6XCJTWVNBRFwiLFwiZWxsaXBzZVVybFwiOlwiaHR0cDovL2VsbGlwc2UtZGVtby5lbXMtZWxsdHN0LmVtcy5jby5pZFwiLFwiZGlzdHJpY3RDb2RlXCI6XCJTRVJWXCJ9IiwibmJmIjoxNjgyMDQyNjE3LCJleHAiOjE2ODIwNTM0MTcsImlhdCI6MTY4MjA0MjYxN30.YRaOA-_V9LqHbrTM2QStWGcRa3BgbJII79jmecLeFuE</value>
         </entry>
      </authorizationInfo>
      <authorizationType>Bearer</authorizationType>
   </authorizationRequest>
   <connectionTimeout>-1</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <httpHeaderProperties>
      <isSelected>true</isSelected>
      <matchCondition>equals</matchCondition>
      <name>Authorization</name>
      <type>Main</type>
      <value>Bearer eyJhbGciOiJIUzI1NiIsInR5cCI6IkpXVCJ9.eyJ1bmlxdWVfbmFtZSI6IkFETUlOIiwicm9sZSI6IkFkbWluaXN0YXJ0b3IiLCJodHRwOi8vc2NoZW1hcy5taWNyb3NvZnQuY29tL3dzLzIwMDgvMDYvaWRlbnRpdHkvY2xhaW1zL3VzZXJkYXRhIjoie1widXNlcl9uYW1lXCI6XCJBRE1JTlwiLFwicGFzc3dvcmRcIjpcIkFkbWluXCIsXCJwb3NpdGlvbklkXCI6XCJTWVNBRFwiLFwiZWxsaXBzZVVybFwiOlwiaHR0cDovL2VsbGlwc2UtZGVtby5lbXMtZWxsdHN0LmVtcy5jby5pZFwiLFwiZGlzdHJpY3RDb2RlXCI6XCJTRVJWXCJ9IiwibmJmIjoxNjgyMDQyNjE3LCJleHAiOjE2ODIwNTM0MTcsImlhdCI6MTY4MjA0MjYxN30.YRaOA-_V9LqHbrTM2QStWGcRa3BgbJII79jmecLeFuE</value>
      <webElementGuid>4dec605a-e67b-4711-8994-9adbc4738e07</webElementGuid>
   </httpHeaderProperties>
   <katalonVersion>8.5.5</katalonVersion>
   <maxResponseSize>-1</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>https://ellipseinterface.ems.co.id/api/ApprovalsManager/RetrieveApprovals</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>-1</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <verificationScript>import static org.assertj.core.api.Assertions.*

import com.kms.katalon.core.testobject.RequestObject
import com.kms.katalon.core.testobject.ResponseObject
import com.kms.katalon.core.webservice.keyword.WSBuiltInKeywords as WS
import com.kms.katalon.core.webservice.verification.WSResponseManager

import groovy.json.JsonSlurper
import internal.GlobalVariable as GlobalVariable

RequestObject request = WSResponseManager.getInstance().getCurrentRequest()

ResponseObject response = WSResponseManager.getInstance().getCurrentResponse()</verificationScript>
   <wsdlAddress></wsdlAddress>
</WebServiceRequestEntity>
