<?xml version="1.0" encoding="UTF-8"?>
<WebServiceRequestEntity>
   <description></description>
   <name>GET List Users</name>
   <tag></tag>
   <elementGuidId>ef3b9a58-37e8-4e33-9462-f5c5b8eb0c31</elementGuidId>
   <selectorMethod>BASIC</selectorMethod>
   <useRalativeImagePath>false</useRalativeImagePath>
   <connectionTimeout>0</connectionTimeout>
   <followRedirects>false</followRedirects>
   <httpBody></httpBody>
   <httpBodyContent></httpBodyContent>
   <httpBodyType></httpBodyType>
   <katalonVersion>8.5.2</katalonVersion>
   <maxResponseSize>0</maxResponseSize>
   <migratedVersion>5.4.1</migratedVersion>
   <restRequestMethod>GET</restRequestMethod>
   <restUrl>${BaseURL}${GETListUsers}?${page}=${pagevalue}</restUrl>
   <serviceType>RESTful</serviceType>
   <soapBody></soapBody>
   <soapHeader></soapHeader>
   <soapRequestMethod></soapRequestMethod>
   <soapServiceEndpoint></soapServiceEndpoint>
   <soapServiceFunction></soapServiceFunction>
   <socketTimeout>0</socketTimeout>
   <useServiceInfoFromWsdl>true</useServiceInfoFromWsdl>
   <variables>
      <defaultValue>GlobalVariable.BaseURL</defaultValue>
      <description></description>
      <id>8b573137-5ac1-4ba8-b780-1e22ad2ede32</id>
      <masked>false</masked>
      <name>BaseURL</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.GETListUsers</defaultValue>
      <description></description>
      <id>ec126f43-48fb-4ed4-adff-869718efed6e</id>
      <masked>false</masked>
      <name>GETListUsers</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.page</defaultValue>
      <description></description>
      <id>3ff43890-26a1-4d75-8cda-b25ae6c03d30</id>
      <masked>false</masked>
      <name>page</name>
   </variables>
   <variables>
      <defaultValue>GlobalVariable.pagevalue</defaultValue>
      <description></description>
      <id>9de3ac2c-ec63-4af4-8033-545539059757</id>
      <masked>false</masked>
      <name>pagevalue</name>
   </variables>
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
